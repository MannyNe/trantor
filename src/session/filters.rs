use std::sync::Arc;

use warp::Filter;

use super::handlers::{self, Event, SessionEnd, SessionStart, SourceName};
use crate::db::{with_db, DB};

pub fn make_session_routes(
    db: DB,
    ua_parser: uaparser::UserAgentParser,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let source_id = with_db(db.clone())
        .and(warp::query::<SourceName>())
        .and_then(handlers::extract_source_id);

    let ua_parser = Arc::new(ua_parser);
    let ua_parser_filter = warp::any().map(move || ua_parser.clone());

    let visitor_id = with_db(db.clone())
        .and(warp::cookie::optional::<String>("visitorId"))
        .and(source_id)
        .and(warp::header::<String>("user-agent"))
        .and(warp::header::<String>("referer"))
        .and(ua_parser_filter)
        .and_then(handlers::extract_visitor_id);

    let session_start = warp::path!("start")
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(visitor_id)
        .and(warp::body::json::<SessionStart>())
        .and_then(handlers::session_start);

    let session_id =
        warp::cookie::optional::<String>("sessionId").and_then(handlers::extract_session_id);

    let session_end = warp::path!("end")
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(session_id)
        .and(warp::body::json::<SessionEnd>())
        .and_then(handlers::session_end);

    let session_event = warp::path!("event")
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(session_id)
        .and(warp::body::json::<Event>())
        .and_then(handlers::session_event);

    warp::path("session").and(session_start.or(session_end).or(session_event))
}
