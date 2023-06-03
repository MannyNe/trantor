use serde::{Deserialize, Serialize};

use crate::{
    db::{
        NewTrackingData, NewUserData, SessionCountByHour, SessionCountByWeekday, SingleSource,
        SingleTracking, SingleVisitor, VisitorCountByBrowser, VisitorCountByDevice,
        VisitorCountByHour, VisitorCountByOs, VisitorCountByWeekday, DB,
    },
    errors::DatabaseError,
};

#[derive(Serialize)]
struct CountVisitorsResponse {
    count: Option<i64>,
}

pub async fn count_visitors(db: DB) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Counting visitors");

    let count = db.count_visitors().await.map_err(|e| {
        log::error!("Error counting visitors: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    Ok(warp::reply::json(&CountVisitorsResponse { count }))
}

#[derive(Deserialize)]
pub struct CreateSourceRequest {
    name: String,
}

pub async fn create_source(
    db: DB,
    request: CreateSourceRequest,
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
    direct_visitors: Option<i64>,
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

#[derive(Deserialize)]
pub struct CreateUserRequest {
    secret_code: String,
}

pub async fn create_user(
    db: DB,
    request: CreateUserRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Creating user");

    let new_user = NewUserData::new(&request.secret_code);

    let user = db.create_user(&new_user).await.map_err(|e| {
        log::error!("Error creating user: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    Ok(warp::reply::with_status(
        warp::reply::json(&user),
        warp::http::StatusCode::CREATED,
    ))
}

pub async fn authenticate_user() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply())
}

#[derive(Deserialize)]
pub struct CreateTrackingRequest {
    name: String,
}

pub async fn create_tracking(
    (db, user_id): (DB, i32),
    req: CreateTrackingRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Creating tracking");

    let new_tracking = NewTrackingData::new(req.name, user_id);
    db.create_tracking(&new_tracking).await.map_err(|e| {
        log::error!("Error creating tracking: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    Ok(warp::reply())
}

#[derive(Serialize)]
struct TrackingStatsResponse {
    trackings: Vec<SingleTracking>,
}

pub async fn list_trackings((db, user_id): (DB, i32)) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Listing trackings");

    let trackings = db.list_trackings(user_id).await.map_err(|e| {
        log::error!("Error listing trackings: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    Ok(warp::reply::json(&TrackingStatsResponse { trackings }))
}

#[derive(Serialize)]
pub struct TrackingResponse {
    name: String,
    session_count_by_weekday: Vec<SessionCountByWeekday>,
    visitor_count_by_weekday: Vec<VisitorCountByWeekday>,
    session_count_by_hour: Vec<SessionCountByHour>,
    visitor_count_by_hour: Vec<VisitorCountByHour>,
    visitor_count_by_os: Vec<VisitorCountByOs>,
    visitor_count_by_browser: Vec<VisitorCountByBrowser>,
    visitor_count_by_device: Vec<VisitorCountByDevice>,
}

pub async fn get_tracking(
    (db, tracking_id): (DB, i32),
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Getting tracking");

    let tracking_name = db.tracking_name(tracking_id).await.map_err(|e| {
        log::error!("Error getting tracking name: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    let session_count_by_weekday =
        db.count_sessions_by_weekday(tracking_id)
            .await
            .map_err(|e| {
                log::error!("Error counting sessions: {}", e);
                warp::reject::custom(DatabaseError)
            })?;
    let visitor_count_by_weekday =
        db.count_visitors_by_weekday(tracking_id)
            .await
            .map_err(|e| {
                log::error!("Error counting visitors: {}", e);
                warp::reject::custom(DatabaseError)
            })?;

    let session_count_by_hour = db.count_sessions_by_hour(tracking_id).await.map_err(|e| {
        log::error!("Error counting sessions by hour: {}", e);
        warp::reject::custom(DatabaseError)
    })?;
    let visitor_count_by_hour = db.count_visitors_by_hour(tracking_id).await.map_err(|e| {
        log::error!("Error counting visitors by hour: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    let visitor_count_by_os = db.count_visitors_by_os(tracking_id).await.map_err(|e| {
        log::error!("Error counting visitors by os: {}", e);
        warp::reject::custom(DatabaseError)
    })?;
    let visitor_count_by_browser =
        db.count_visitors_by_browser(tracking_id)
            .await
            .map_err(|e| {
                log::error!("Error counting visitors by browser: {}", e);
                warp::reject::custom(DatabaseError)
            })?;
    let visitor_count_by_device = db
        .count_visitors_by_device(tracking_id)
        .await
        .map_err(|e| {
            log::error!("Error counting visitors by device: {}", e);
            warp::reject::custom(DatabaseError)
        })?;

    Ok(warp::reply::json(&TrackingResponse {
        name: tracking_name,
        session_count_by_weekday,
        visitor_count_by_weekday,
        session_count_by_hour,
        visitor_count_by_hour,
        visitor_count_by_os,
        visitor_count_by_browser,
        visitor_count_by_device,
    }))
}

#[derive(Serialize)]
struct ListVisitorsResponse {
    visitors: Vec<SingleVisitor>,
}

pub async fn list_visitors(
    (db, tracking_id): (DB, i32),
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Listing visitors");

    let visitors = db.list_visitors(tracking_id).await.map_err(|e| {
        log::error!("Error listing visitors: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    Ok(warp::reply::json(&ListVisitorsResponse { visitors }))
}
