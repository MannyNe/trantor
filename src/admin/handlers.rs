use serde::{Deserialize, Serialize};

use crate::{
    db::{SingleSource, SingleVisitor, DB},
    errors::DatabaseError,
};

#[derive(Serialize)]
struct CountVisitorsResponse {
    count: i64,
}

pub async fn count_visitors(db: DB) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Counting visitors");

    let count = db.count_visitors().await.map_err(|e| {
        log::error!("Error counting visitors: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    Ok(warp::reply::json(&CountVisitorsResponse { count }))
}

#[derive(Serialize)]
struct ListVisitorsResponse {
    visitors: Vec<SingleVisitor>,
}

pub async fn list_visitors(db: DB) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Listing visitors");

    let visitors = db.list_visitors().await.map_err(|e| {
        log::error!("Error listing visitors: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    Ok(warp::reply::json(&ListVisitorsResponse { visitors }))
}

#[derive(Deserialize)]
pub struct CreateSourceRequest {
    name: String,
}

pub async fn create_source(
    request: CreateSourceRequest,
    db: DB,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Creating source: {}", request.name);

    db.create_source(&request.name).await.map_err(|e| {
        log::error!("Error creating source: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    Ok(warp::reply::with_status(
        warp::reply(),
        warp::http::StatusCode::CREATED,
    ))
}

#[derive(Serialize)]
struct ListSourcesResponse {
    sources: Vec<SingleSource>,
    direct_visitors: i64,
}

pub async fn list_sources(db: DB) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Listing sources");

    let sources = db.list_sources().await.map_err(|e| {
        log::error!("Error listing sources: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    let visitors_without_source = db.count_visitors_without_source().await.map_err(|e| {
        log::error!("Error counting visitors without source: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    Ok(warp::reply::json(&ListSourcesResponse {
        sources,
        direct_visitors: visitors_without_source,
    }))
}

pub async fn list_sessions(db: DB) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Listing sessions");

    let sessions = db.list_sessions().await.map_err(|e| {
        log::error!("Error listing sessions: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    Ok(warp::reply::json(&sessions))
}
