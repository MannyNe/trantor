use serde::{Deserialize, Serialize};

use crate::{
    db::{SingleVisitor, DB},
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
