use domain::{serde, Service, SessionsRepository};
use services::{SessionEndError, SessionEndRequest, SessionEndResponse, SessionEndService};

use crate::warp_service;
use warp::{
    http::{Response, StatusCode},
    Filter,
};

#[derive(serde::Deserialize)]
#[serde(crate = "domain::serde")]
struct SessionEnd {
    timestamp: f64,
}

fn extract_session_end_request(
) -> impl warp::Filter<Extract = (SessionEndRequest,), Error = warp::Rejection> + Clone {
    warp::header("x-tracking-id")
        .and(warp::cookie("session_id"))
        .and(warp::body::json::<SessionEnd>())
        .map(make_request)
}

fn make_request(
    tracking_id: String,
    session_id: String,
    session_end: SessionEnd,
) -> SessionEndRequest {
    SessionEndRequest::new(tracking_id, session_id, session_end.timestamp)
}

pub(crate) fn session_end_filter<SR>(
    service: SessionEndService<SR>,
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    SR: SessionsRepository + Clone + Send + Sync,
{
    warp_service(service)
        .and(extract_session_end_request())
        .and_then(session_end_handler)
}

async fn session_end_handler<SR>(
    service: SessionEndService<SR>,
    request: SessionEndRequest,
) -> Result<impl warp::Reply, std::convert::Infallible>
where
    SR: SessionsRepository + Clone + Send + Sync,
{
    Ok(match service.execute(request).await {
        Ok(resp) => make_session_end_response(resp),
        Err(err) => make_session_end_error_response(err),
    })
}

fn make_session_end_response(_: SessionEndResponse) -> warp::reply::Response {
    Response::builder()
        .status(StatusCode::OK)
        .body(warp::hyper::Body::empty())
        .expect("failed to create session end response")
}

fn make_session_end_error_response(_: SessionEndError) -> warp::reply::Response {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(warp::hyper::Body::empty())
        .expect("failed to create session end error response")
}
