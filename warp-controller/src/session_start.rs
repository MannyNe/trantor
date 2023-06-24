use domain::{
    serde, GeoIpReader, Service, SessionsRepository, UserAgentParserPort, VisitorsRepository,
};
use services::{SessionStartError, SessionStartRequest, SessionStartResponse, SessionStartService};

use crate::warp_service;
use std::net::SocketAddr;
use warp::{
    http::{Response, StatusCode},
    Filter,
};

#[derive(serde::Deserialize)]
#[serde(crate = "domain::serde")]
struct SessionStart {
    timestamp: f64,
    title: String,
    pathname: String,
    referral: Option<String>,
}

fn extract_session_start_request(
) -> impl warp::Filter<Extract = (SessionStartRequest,), Error = warp::Rejection> + Clone {
    warp::header("x-tracking-id")
        .and(warp::header::optional("x-source-name"))
        .and(warp::header::optional("x-visitor-id"))
        .and(warp::header("user-agent"))
        .and(warp::header("referer"))
        .and(warp::addr::remote())
        .and(warp::body::json::<SessionStart>())
        .map(make_request)
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
        .expect("remote_addr should always be present");

    SessionStartRequest::new(
        tracking_id,
        source_name,
        visitor_id,
        remote_ip,
        user_agent,
        referer,
        session_start.timestamp,
        session_start.title,
        session_start.pathname,
        session_start.referral,
    )
}

fn make_start_session_response(resp: SessionStartResponse) -> warp::reply::Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Set-Cookie", format!("visitor_id={}", resp.visitor_id()))
        .header("Set-Cookie", format!("session_id={}", resp.session_id()))
        .body(warp::hyper::Body::empty())
        .expect("failed to create session start response")
}

fn make_start_session_error_response(_: SessionStartError) -> warp::reply::Response {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(warp::hyper::Body::empty())
        .unwrap()
}

pub fn session_start_filter<SR, VR, UAP, GIR>(
    service: SessionStartService<SR, VR, UAP, GIR>,
) -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
where
    SR: SessionsRepository + Clone + Send + Sync,
    VR: VisitorsRepository + Clone + Send + Sync,
    UAP: UserAgentParserPort + Clone + Send + Sync,
    GIR: GeoIpReader + Clone + Send + Sync,
{
    warp_service(service)
        .and(extract_session_start_request())
        .and_then(session_start_handler)
}

async fn session_start_handler<SR, VR, UAP, GIR>(
    service: SessionStartService<SR, VR, UAP, GIR>,
    request: SessionStartRequest,
) -> Result<impl warp::Reply, std::convert::Infallible>
where
    SR: SessionsRepository + Clone + Send + Sync,
    VR: VisitorsRepository + Clone + Send + Sync,
    UAP: UserAgentParserPort + Clone + Send + Sync,
    GIR: GeoIpReader + Clone + Send + Sync,
{
    Ok(match service.execute(request).await {
        Ok(resp) => make_start_session_response(resp),
        Err(err) => make_start_session_error_response(err),
    })
}
