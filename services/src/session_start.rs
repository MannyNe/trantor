use domain::{
    async_trait::async_trait, thiserror, GeoIpReader, GeoIpReaderError, Service, Session,
    SessionRepositoryError, SessionsRepository, UserAgentParserError, UserAgentParserPort, Visitor,
    VisitorRepositoryError, VisitorsRepository,
};

#[derive(Clone)]
pub struct SessionStartService<SR, VR, UAP, GIR> {
    sessions: SR,
    visitors: VR,
    user_agent_parser: UAP,
    geo_ip_reader: GIR,
}

impl<SR, VR, UAP, GIR> SessionStartService<SR, VR, UAP, GIR>
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
}

#[async_trait]
impl<SR, VR, UAP, GIR> Service for SessionStartService<SR, VR, UAP, GIR>
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
                    Visitor::new(&req.tracking_id, req.source_name, req.referer, user_agent);
                self.visitors.create(&visitor).await?
            }
        };

        let location = self.geo_ip_reader.parse(req.remote_ip).await?;
        let session = Session::new(
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

pub struct SessionStartRequest {
    tracking_id: String,
    source_name: Option<String>,
    visitor_id: Option<String>,
    remote_ip: std::net::IpAddr,
    user_agent: String,
    referer: String,
    timestamp: f64,
    title: String,
    pathname: String,
    referral: Option<String>,
}

impl SessionStartRequest {
    pub fn new(
        tracking_id: String,
        source_name: Option<String>,
        visitor_id: Option<String>,
        remote_ip: std::net::IpAddr,
        user_agent: String,
        referer: String,
        timestamp: f64,
        title: String,
        pathname: String,
        referral: Option<String>,
    ) -> Self {
        Self {
            tracking_id,
            source_name,
            visitor_id,
            remote_ip,
            user_agent,
            referer,
            timestamp,
            title,
            pathname,
            referral,
        }
    }
}

pub struct SessionStartResponse {
    visitor_id: String,
    session_id: String,
}

impl SessionStartResponse {
    pub fn visitor_id(&self) -> &str {
        &self.visitor_id
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }
}
