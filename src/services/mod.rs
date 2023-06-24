use async_trait::async_trait;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod adapters;
mod models;
mod ports;
use ports::{GeoIpReader, SessionsRepository, UseCase, UserAgentParserPort, VisitorsRepository};

use crate::services::adapters::{
    MaxmindGeoIpReader, PgSessionsRepository, PgVisitorsRepository, UAParser,
};

use self::{
    models::{SessionModel, VisitorModel},
    ports::{
        GeoIpReaderError, SessionRepositoryError, UserAgentParserError, VisitorRepositoryError,
    },
};

#[derive(Clone)]
pub struct SessionStartUseCase<SR, VR, UAP, GIR> {
    sessions: SR,
    visitors: VR,
    user_agent_parser: UAP,
    geo_ip_reader: GIR,
}

impl<SR, VR, UAP, GIR> SessionStartUseCase<SR, VR, UAP, GIR>
where
    SR: SessionsRepository + Clone + Send,
    VR: VisitorsRepository + Clone + Send,
    UAP: UserAgentParserPort + Clone + Send,
    GIR: GeoIpReader + Clone + Send,
{
    pub fn new(sessions: SR, visitors: VR, user_agent_parser: UAP, geo_ip_reader: GIR) -> Self {
        Self {
            sessions,
            visitors,
            user_agent_parser,
            geo_ip_reader,
        }
    }

    pub fn handle(
        self,
    ) -> impl warp::Filter<Extract = (Self,), Error = std::convert::Infallible> + Clone {
        use warp::Filter;
        warp::any().map(move || self.clone())
    }
}

#[async_trait]
impl<SR, VR, UAP, GIR> UseCase for SessionStartUseCase<SR, VR, UAP, GIR>
where
    SR: SessionsRepository + Sync + Send + Clone,
    VR: VisitorsRepository + Sync + Send + Clone,
    UAP: UserAgentParserPort + Sync + Send + Clone,
    GIR: GeoIpReader + Sync + Send + Clone,
{
    type Error = SessionStartError;
    type Request = SessionStartRequest;
    type Response = SessionStartResponse;

    async fn execute(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        let visitor_id = match req.visitor_id {
            Some(visitor_id) if self.visitors.exists(&visitor_id).await? => visitor_id,
            _ => {
                let user_agent = self.user_agent_parser.parse(&req.user_agent).await?;
                let visitor =
                    VisitorModel::new(&req.tracking_id, req.source_name, req.referer, user_agent);
                self.visitors.create(&visitor).await?
            }
        };

        let location = self.geo_ip_reader.parse(req.remote_ip).await?;
        let session = SessionModel::new(
            &req.tracking_id,
            visitor_id.to_owned(),
            req.timestamp,
            req.title,
            req.pathname,
            req.referral,
            location,
        );
        let session_id = self.sessions.create(&session).await?;

        Ok(SessionStartResponse {
            visitor_id,
            session_id,
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SessionStartError {
    #[error("error in sessions repository")]
    SessionsRepository(#[from] SessionRepositoryError),
    #[error("error in visitors repository")]
    VisitorsRepository(#[from] VisitorRepositoryError),
    #[error("error in user agent parser")]
    UserAgentParser(#[from] UserAgentParserError),
    #[error("error in geo ip reader")]
    GeoIpReader(#[from] GeoIpReaderError),
}

impl warp::reply::Reply for SessionStartError {
    fn into_response(self) -> warp::reply::Response {
        warp::http::Response::builder()
            .status(warp::http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(warp::hyper::Body::empty())
            .unwrap()
    }
}

pub struct SessionStartRequest {
    tracking_id: String,
    source_name: Option<String>,
    visitor_id: Option<String>,
    remote_ip: IpAddr,
    user_agent: String,
    referer: String,
    timestamp: f64,
    title: String,
    pathname: String,
    referral: Option<String>,
}

impl SessionStartRequest {
    pub fn extract() -> impl warp::Filter<Extract = (Self,), Error = warp::Rejection> + Clone {
        #[derive(serde::Deserialize)]
        pub struct SessionStart {
            timestamp: f64,
            title: String,
            pathname: String,
            referral: Option<String>,
        }

        fn make_request(
            tracking_id: String,
            source_name: Option<String>,
            visitor_id: Option<String>,
            user_agent: String,
            referer: String,
            remote_addr: Option<SocketAddr>,
            session_start: SessionStart,
        ) -> SessionStartRequest {
            let remote_ip = remote_addr
                .map(|addr| addr.ip())
                .unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));

            SessionStartRequest {
                tracking_id,
                source_name,
                visitor_id,
                remote_ip,
                user_agent,
                referer,
                timestamp: session_start.timestamp,
                title: session_start.title,
                pathname: session_start.pathname,
                referral: session_start.referral,
            }
        }

        use warp::Filter;

        warp::header("x-tracking-id")
            .and(warp::header::optional("x-source-name"))
            .and(warp::header::optional("x-visitor-id"))
            .and(warp::header("user-agent"))
            .and(warp::header("referer"))
            .and(warp::addr::remote())
            .and(warp::body::json::<SessionStart>())
            .map(make_request)
    }
}

pub struct SessionStartResponse {
    visitor_id: String,
    session_id: String,
}

impl warp::reply::Reply for SessionStartResponse {
    fn into_response(self) -> warp::reply::Response {
        warp::http::Response::builder()
            .status(warp::http::StatusCode::OK)
            .header("Set-Cookie", format!("visitor_id={}", self.visitor_id))
            .header("Set-Cookie", format!("session_id={}", self.session_id))
            .body(warp::hyper::Body::empty())
            .expect("failed to create session start response")
    }
}

pub fn session_start_handler<SR, VR, UAP, GIR>(
    usecase: SessionStartUseCase<SR, VR, UAP, GIR>,
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    SR: SessionsRepository + Clone + Send + Sync,
    VR: VisitorsRepository + Clone + Send + Sync,
    UAP: UserAgentParserPort + Clone + Send + Sync,
    GIR: GeoIpReader + Clone + Send + Sync,
{
    use warp::Filter;

    usecase
        .handle()
        .and(SessionStartRequest::extract())
        .and_then(create_event)
}

async fn create_event<SR, VR, UAP, GIR>(
    usecase: SessionStartUseCase<SR, VR, UAP, GIR>,
    request: SessionStartRequest,
) -> Result<impl warp::Reply, std::convert::Infallible>
where
    SR: SessionsRepository + Clone + Send + Sync,
    VR: VisitorsRepository + Clone + Send + Sync,
    UAP: UserAgentParserPort + Clone + Send + Sync,
    GIR: GeoIpReader + Clone + Send + Sync,
{
    use warp::Reply;

    Ok(match usecase.execute(request).await {
        Ok(response) => response.into_response(),
        Err(err) => err.into_response(),
    })
}

pub fn session_start_route(
    pool: sqlx::PgPool,
    uaparser: uaparser::UserAgentParser,
    maxminddb: maxminddb::Reader<Vec<u8>>,
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    use warp::Filter;

    let sessions = PgSessionsRepository::new(pool.clone());
    let visitors = PgVisitorsRepository::new(pool.clone());
    let user_agent_parser = UAParser::new(uaparser);
    let geo_ip_reader = MaxmindGeoIpReader::new(maxminddb);

    let usecase = SessionStartUseCase::new(sessions, visitors, user_agent_parser, geo_ip_reader);

    warp::path!("session" / "start").and(session_start_handler(usecase))
}
