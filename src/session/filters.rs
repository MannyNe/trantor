use std::sync::Arc;

use warp::Filter;

use super::handlers::{self, Event, SessionEnd, SessionStart};
use crate::db::{with_db, DB};

pub fn make_session_routes(
    db: DB,
    ua_parser: uaparser::UserAgentParser,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let ua_parser = Arc::new(ua_parser);
    let ua_parser_filter = warp::any().map(move || ua_parser.clone());

    let visitor_id = with_db(db.clone())
        .and(warp::header("x-tracking-id"))
        .and_then(handlers::extract_tracking_id)
        .and(warp::header::optional::<String>("x-source-name"))
        .and_then(|(db, tracking_id), source_id| async move {
            let (db, source_id) = handlers::extract_source_id(db, tracking_id, source_id).await?;
            Ok::<_, warp::Rejection>((db, source_id, tracking_id))
        })
        .and(warp::cookie::optional("visitorId"))
        .and(warp::header("user-agent"))
        .and(warp::header("referer"))
        .and(ua_parser_filter)
        .and_then(
            |(db, source_id, tracking_id), visitor_id, user_agent, referer, ua_parser| async move {
                let visitor_id = handlers::extract_visitor_id(
                    db,
                    source_id,
                    tracking_id,
                    visitor_id,
                    user_agent,
                    referer,
                    ua_parser,
                )
                .await?;
                Ok::<_, warp::Rejection>(visitor_id)
            },
        );

    let session_start = warp::path!("start")
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(warp::header("x-tracking-id"))
        .and_then(|db, tracking_id| async move {
            let (db, tracking_id) = handlers::extract_tracking_id(db, tracking_id).await?;
            Ok::<_, warp::Rejection>((db, tracking_id))
        })
        .and(visitor_id)
        .and(warp::body::json::<SessionStart>())
        .and_then(|(db, tracking_id), visitor_id, session_start| async move {
            let reply = handlers::session_start(db, tracking_id, visitor_id, session_start).await?;
            Ok::<_, warp::Rejection>(reply)
        });

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
        .and(with_db(db))
        .and(warp::header("x-tracking-id"))
        .and_then(|db, tracking_id| async move {
            let (db, tracking_id) = handlers::extract_tracking_id(db, tracking_id).await?;
            Ok::<_, warp::Rejection>((db, tracking_id))
        })
        .and(session_id)
        .and(warp::body::json::<Event>())
        .and_then(|(db, tracking_id), session_id, event| async move {
            let reply = handlers::session_event(db, session_id, event, tracking_id).await?;
            Ok::<_, warp::Rejection>(reply)
        });

    warp::path("session").and(session_start.or(session_end).or(session_event))
}
