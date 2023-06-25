mod session_start;

use std::convert::Infallible;

use services::SessionStartService;
use session_start::session_start_filter;

use domain::Service;
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

        let session_start = warp::path("start").and(session_start_filter(self.session_start));
        let session_end = warp::path!("end").and(warp::any().map(|| "OK"));
        let session_routes = warp::path!("sessions").and(session_start.or(session_end));
        let analytics_routes = warp::path!("analytics").map(|| "OK");

        session_routes
            .or(analytics_routes)
            .recover(recover)
            .with(cors)
    }
}

async fn recover(_: Rejection) -> Result<impl Reply, Infallible> {
    Ok(warp::reply::with_status(
        warp::reply::json(&"Internal Server Error"),
        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
    ))
}
