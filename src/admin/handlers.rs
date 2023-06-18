use serde::{Deserialize, Serialize};

use crate::{
    db::{
        CountByBrowser, CountByCountry, CountByDevice, CountByHour, CountByOs, CountByPathname,
        CountByReferral, CountByTitle, CountByWeekday, NewTrackingData, NewUserData, SingleReferer,
        SingleSource, SingleTracking, DB,
    },
    errors::DatabaseError,
};

// User Routes

pub async fn authenticate_user() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply())
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    secret_code: String,
}

pub async fn create_user(
    db: DB,
    request: CreateUserRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    tracing::info!("Creating user");

    let new_user = NewUserData::new(&request.secret_code);

    let user = db.create_user(&new_user).await.map_err(|e| {
        tracing::error!("Error creating user: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    Ok(warp::reply::with_status(
        warp::reply::json(&user),
        warp::http::StatusCode::CREATED,
    ))
}

// Tracking Routes

#[derive(Deserialize)]
pub struct CreateTrackingRequest {
    name: String,
}

pub async fn create_tracking(
    (db, user_id): (DB, i32),
    req: CreateTrackingRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    tracing::info!("Creating tracking");

    let new_tracking = NewTrackingData::new(req.name, user_id);
    db.create_tracking(&new_tracking).await.map_err(|e| {
        tracing::error!("Error creating tracking: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    Ok(warp::reply())
}

#[derive(Serialize)]
struct TrackingsResponse {
    trackings: Vec<SingleTracking>,
}

pub async fn list_trackings((db, user_id): (DB, i32)) -> Result<impl warp::Reply, warp::Rejection> {
    tracing::info!("Listing trackings");

    let trackings = db.list_trackings(user_id).await.map_err(|e| {
        tracing::error!("Error listing trackings: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    Ok(warp::reply::json(&TrackingsResponse { trackings }))
}

#[derive(Serialize)]
pub struct TrackingResponse {
    name: String,
    session_count_by_weekday: Vec<CountByWeekday>,
    visitor_count_by_weekday: Vec<CountByWeekday>,

    session_count_by_hour: Vec<CountByHour>,
    visitor_count_by_hour: Vec<CountByHour>,

    visitor_count_by_os: Vec<CountByOs>,
    visitor_count_by_browser: Vec<CountByBrowser>,
    visitor_count_by_device: Vec<CountByDevice>,
}

pub async fn get_tracking(db: DB, tracking_id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    tracing::info!("Getting tracking");

    let tracking_name = db.tracking_name(tracking_id).await.map_err(|e| {
        tracing::error!("Error getting tracking name: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    let session_count_by_weekday =
        db.count_sessions_by_weekday(tracking_id)
            .await
            .map_err(|e| {
                tracing::error!("Error counting sessions: {}", e);
                warp::reject::custom(DatabaseError)
            })?;
    let visitor_count_by_weekday =
        db.count_visitors_by_weekday(tracking_id)
            .await
            .map_err(|e| {
                tracing::error!("Error counting visitors: {}", e);
                warp::reject::custom(DatabaseError)
            })?;

    let session_count_by_hour = db.count_sessions_by_hour(tracking_id).await.map_err(|e| {
        tracing::error!("Error counting sessions by hour: {}", e);
        warp::reject::custom(DatabaseError)
    })?;
    let visitor_count_by_hour = db.count_visitors_by_hour(tracking_id).await.map_err(|e| {
        tracing::error!("Error counting visitors by hour: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    let visitor_count_by_os = db.count_visitors_by_os(tracking_id).await.map_err(|e| {
        tracing::error!("Error counting visitors by os: {}", e);
        warp::reject::custom(DatabaseError)
    })?;
    let visitor_count_by_browser =
        db.count_visitors_by_browser(tracking_id)
            .await
            .map_err(|e| {
                tracing::error!("Error counting visitors by browser: {}", e);
                warp::reject::custom(DatabaseError)
            })?;
    let visitor_count_by_device = db
        .count_visitors_by_device(tracking_id)
        .await
        .map_err(|e| {
            tracing::error!("Error counting visitors by device: {}", e);
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

#[derive(Deserialize)]
pub struct RenameTrackingRequest {
    name: String,
}

pub async fn rename_tracking(
    db: DB,
    tracking_id: i32,
    req: RenameTrackingRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    tracing::info!("Renaming tracking: {}", tracking_id);

    db.rename_tracking(tracking_id, &req.name)
        .await
        .map_err(|e| {
            tracing::error!("Error renaming tracking: {}", e);
            warp::reject::custom(DatabaseError)
        })?;

    Ok(warp::reply::with_status(
        warp::reply(),
        warp::http::StatusCode::NO_CONTENT,
    ))
}

pub async fn delete_tracking(
    db: DB,
    tracking_id: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    tracing::info!("Deleting tracking: {}", tracking_id);

    db.delete_tracking(tracking_id).await.map_err(|e| {
        tracing::error!("Error deleting tracking: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    Ok(warp::reply::with_status(
        warp::reply(),
        warp::http::StatusCode::NO_CONTENT,
    ))
}

#[derive(Serialize)]
struct TrackingCountsResponse {
    sources: Vec<SingleSource>,
    paths: Vec<CountByPathname>,
    titles: Vec<CountByTitle>,
    refers: Vec<SingleReferer>,
    countries: Vec<CountByCountry>,
    referrals: Vec<CountByReferral>,
}

pub async fn tracking_counts(
    db: DB,
    tracking_id: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    tracing::info!("Getting tracking counts: {}", tracking_id);

    let mut sources = db.list_sources(tracking_id).await.map_err(|e| {
        tracing::error!("Error listing sources: {}", e);
        warp::reject::custom(DatabaseError)
    })?;
    let direct = db
        .visitors_and_sessions_no_source(tracking_id)
        .await
        .map_err(|e| {
            tracing::error!("Error counting direct visitors: {}", e);
            warp::reject::custom(DatabaseError)
        })?;
    sources.push(direct);

    let paths = db
        .count_sessions_by_pathname(tracking_id)
        .await
        .map_err(|e| {
            tracing::error!("Error counting sessions by pathname: {}", e);
            warp::reject::custom(DatabaseError)
        })?;
    let titles = db.count_sessions_by_title(tracking_id).await.map_err(|e| {
        tracing::error!("Error counting sessions by title: {}", e);
        warp::reject::custom(DatabaseError)
    })?;
    let refers = db.list_refers(tracking_id).await.map_err(|e| {
        tracing::error!("Error listing refers: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    let countries = db
        .count_sessions_by_country(tracking_id)
        .await
        .map_err(|e| {
            tracing::error!("Error counting sessions by country: {}", e);
            warp::reject::custom(DatabaseError)
        })?;

    let referrals = db
        .count_sessions_by_referral(tracking_id)
        .await
        .map_err(|e| {
            tracing::error!("Error counting sessions by referral: {}", e);
            warp::reject::custom(DatabaseError)
        })?;

    Ok(warp::reply::json(&TrackingCountsResponse {
        sources,
        paths,
        titles,
        refers,
        countries,
        referrals,
    }))
}

// Source Routes

#[derive(Deserialize)]
pub struct CreateSourceRequest {
    name: String,
}

pub async fn create_source(
    db: DB,
    tracking_id: i32,
    request: CreateSourceRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    tracing::info!("Creating source: {}", request.name);

    db.create_source(&request.name, tracking_id)
        .await
        .map_err(|e| {
            tracing::error!("Error creating source: {}", e);
            warp::reject::custom(DatabaseError)
        })?;

    Ok(warp::reply::with_status(
        warp::reply(),
        warp::http::StatusCode::CREATED,
    ))
}

pub async fn delete_source(
    db: DB,
    tracking_id: i32,
    source_name: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    tracing::info!("Deleting source: {}", source_name);

    db.delete_source(&source_name, tracking_id)
        .await
        .map_err(|e| {
            tracing::error!("Error creating source: {}", e);
            warp::reject::custom(DatabaseError)
        })?;

    Ok(warp::reply::with_status(
        warp::reply(),
        warp::http::StatusCode::NO_CONTENT,
    ))
}
