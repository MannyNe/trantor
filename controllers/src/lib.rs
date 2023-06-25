mod session_start;

use std::convert::Infallible;

use services::SessionStartService;
use session_start::session_start_filter;

use domain::{serde::Serialize, tracing, Service};
use warp::{Filter, Rejection, Reply};

pub use warp;

pub(crate) fn warp_service<S: Service + Clone + Send + Sync>(
    service: S,
) -> impl warp::Filter<Extract = (S,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || service.clone())
}

pub struct Controllers<SR, VR, UAP, GIR> {
    session_start: SessionStartService<SR, VR, UAP, GIR>,
}

impl<SR, VR, UAP, GIR> Controllers<SR, VR, UAP, GIR>
where
    SR: domain::SessionsRepository + Clone + Send + Sync + 'static,
    VR: domain::VisitorsRepository + Clone + Send + Sync + 'static,
    UAP: domain::UserAgentParser + Clone + Send + Sync + 'static,
    GIR: domain::GeoIpReader + Clone + Send + Sync + 'static,
{
    pub fn new(
        session_start: SessionStartService<SR, VR, UAP, GIR>,
    ) -> Controllers<SR, VR, UAP, GIR> {
        Controllers { session_start }
    }

    pub fn routes(
        self,
    ) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        let cors = warp::cors()
            .allow_any_origin()
            .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allow_headers(vec![
                "Origin",
                "Content-Type",
                "x-tracking-id",
                "x-source-name",
                "Authorization",
                "Content-Length",
                "Access-Control-Allow-Origin",
            ])
            .allow_credentials(true);

        let session_start = warp::path("start")
            .and(warp::path::end())
            .and(warp::post())
            .and(session_start_filter(self.session_start));
        let session_end = warp::path("end")
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::any().map(|| "OK"));
        let session_routes = warp::path("sessions").and(session_start.or(session_end));
        let analytics_routes = warp::path("analytics").map(|| "OK");

        session_routes
            .or(analytics_routes)
            .recover(recover)
            .with(cors)
    }
}

#[derive(Serialize)]
#[serde(crate = "domain::serde")]
struct ErrorMessage {
    code: u16,
    message: String,
}

async fn recover(err: Rejection) -> Result<impl Reply, Infallible> {
    use warp::http::StatusCode;

    let code: StatusCode;
    let message: String;

    tracing::error!("unhandled rejection: {:?}", err);

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND".to_owned();
    } else if let Some(err) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = err.clone().to_string();
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED".to_owned();
    } else if err.find::<warp::reject::MissingHeader>().is_some() {
        code = StatusCode::BAD_REQUEST;
        message = "MISSING_HEADER".to_owned();
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION".to_owned();
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
