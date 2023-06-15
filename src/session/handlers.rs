use std::sync::Arc;

use serde::Deserialize;
use warp::{
    hyper::{Body, Response, StatusCode},
    reject,
};

use crate::{
    db::{NewSessionData, NewVisitorData, DB},
    errors::{DatabaseError, MissingSessionId},
};

pub async fn extract_source_id(
    db: DB,
    tracking_id: i32,
    source_name: Option<String>,
) -> Result<(DB, Option<i32>), reject::Rejection> {
    tracing::info!("Extracting source name: {:?}", source_name);
    let id = match source_name {
        Some(source_name) => Some(
            db.id_from_source_name(tracking_id, &source_name)
                .await
                .map_err(|e| {
                    tracing::error!("Error getting source id: {}", e);
                    reject::custom(DatabaseError)
                })?,
        ),
        None => None,
    };

    Ok((db, id))
}

pub async fn extract_tracking_id(
    db: DB,
    tracking_id: String,
) -> Result<(DB, i32), reject::Rejection> {
    let tracking_id = db.id_from_tracking_id(&tracking_id).await.map_err(|e| {
        tracing::error!("Error getting tracking id: {}", e);
        reject::custom(DatabaseError)
    })?;

    Ok((db, tracking_id))
}

pub async fn extract_visitor_id(
    db: DB,
    source_id: Option<i32>,
    tracking_id: i32,
    visitor_id: Option<String>,
    user_agent: String,
    referer: String,
    ua_parser: Arc<uaparser::UserAgentParser>,
) -> Result<(i32, String), reject::Rejection> {
    match visitor_id {
        Some(visitor_id) => {
            let id = db.id_from_visitor_id(&visitor_id).await.map_err(|e| {
                tracing::error!("Error getting visitor id: {}", e);
                reject::custom(DatabaseError)
            })?;

            Ok((id, visitor_id))
        }
        None => {
            let new_visitor =
                NewVisitorData::new(user_agent, referer, source_id, ua_parser, tracking_id);

            let id = db.create_visitor(&new_visitor).await.map_err(|e| {
                tracing::error!("Error creating visitor: {}", e);
                reject::custom(DatabaseError)
            })?;

            Ok((id, new_visitor.visitor_id()))
        }
    }
}

pub async fn extract_session_id(session_id: Option<String>) -> Result<String, reject::Rejection> {
    let session_id = session_id.ok_or_else(|| {
        tracing::error!("Missing session id");
        reject::custom(MissingSessionId)
    })?;

    Ok(session_id)
}

#[derive(Deserialize)]
pub struct SessionStart {
    timestamp: f64,
    title: String,
    pathname: String,
}

pub async fn session_start(
    db: DB,
    tracking_id: i32,
    (visitor_id, visitor_id_public): (i32, String),
    SessionStart {
        timestamp,
        title,
        pathname,
    }: SessionStart,
) -> Result<impl warp::Reply, reject::Rejection> {
    tracing::info!("session-start");
    tracing::info!("visitor_id: {}", visitor_id);
    tracing::info!("timestamp: {}", timestamp);
    tracing::info!("title: {}", title);
    tracing::info!("pathname: {}", pathname);

    let new_session = NewSessionData::new(visitor_id, timestamp, title, pathname, tracking_id);

    db.create_session(&new_session).await.map_err(|e| {
        tracing::error!("Error creating session: {}", e);
        reject::custom(DatabaseError)
    })?;

    let resp = Response::builder()
        .status(StatusCode::OK)
        .header(
            "Set-Cookie",
            format!("visitorId={}; HttpOnly", &visitor_id_public,),
        )
        .header(
            "Set-Cookie",
            format!("sessionId={}; HttpOnly", new_session.session_id()),
        )
        .body(Body::empty())
        .unwrap();

    Ok(resp)
}

#[derive(Deserialize)]
pub struct SessionEnd {
    timestamp: f64,
}

pub async fn session_end(
    db: DB,
    session_id: String,
    SessionEnd { timestamp }: SessionEnd,
) -> Result<impl warp::Reply, reject::Rejection> {
    tracing::info!("session-end");
    tracing::info!("session_id: {}", session_id);
    tracing::info!("timestamp: {}", timestamp);

    db.end_session(&session_id, timestamp).await.map_err(|e| {
        tracing::error!("Error ending session: {}", e);
        reject::custom(DatabaseError)
    })?;

    Ok(warp::reply::with_header(
        warp::reply(),
        "Set-Cookie",
        "sessionId=; HttpOnly; Max-Age=0",
    ))
}

#[derive(Deserialize)]
pub struct Event {
    #[serde(rename = "type")]
    _type: String,
    target: String,
}

pub async fn session_event(
    db: DB,
    session_id: String,
    event: Event,
    tracking_id: i32,
) -> Result<impl warp::Reply, reject::Rejection> {
    tracing::info!("session-event");
    tracing::info!("session_id: {}", session_id);

    db.create_event(&session_id, &event._type, &event.target, tracking_id)
        .await
        .map_err(|e| {
            tracing::error!("Error creating event: {}", e);
            reject::custom(DatabaseError)
        })?;

    Ok(warp::reply())
}
