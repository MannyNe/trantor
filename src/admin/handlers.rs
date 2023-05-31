use serde::{Deserialize, Serialize};

use crate::{
    db::{NewUserData, SingleSource, SingleVisitor, DB},
    errors::{DatabaseError, InvalidBase64, InvalidToken},
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

pub async fn home_page(
    db: DB,
    template: liquid::Template,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Rendering home page");

    let visitors = db.count_visitors().await.map_err(|e| {
        log::error!("Error counting visitors: {}", e);
        warp::reject::custom(DatabaseError)
    })?;
    let sessions = db.count_sessions().await.map_err(|e| {
        log::error!("Error counting sessions: {}", e);
        warp::reject::custom(DatabaseError)
    })?;
    let sources = db.count_sources().await.map_err(|e| {
        log::error!("Error listing sources: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    let body = template
        .render(&liquid::object!({
            "visitors": visitors,
            "sessions": sessions,
            "sources": sources,
        }))
        .unwrap();

    Ok(warp::reply::html(body))
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

pub async fn authenticate_user(db: DB, token: String) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Authenticating user with token: {}", token);

    let engine = base64::engine::general_purpose::URL_SAFE;

    use base64::Engine;

    let decoded = engine.decode(token).map_err(|e| {
        log::error!("Error decoding token: {}", e);
        warp::reject::custom(InvalidBase64)
    })?;
    let decode = String::from_utf8(decoded).map_err(|e| {
        log::error!("Error decoding token to UTF-8: {}", e);
        warp::reject::custom(InvalidBase64)
    })?;

    let (user_id, secret_code) = decode.split_once(':').ok_or_else(|| {
        log::error!("Error splitting token into user ID and secret code");
        warp::reject::custom(InvalidBase64)
    })?;

    let secret_code_from_db = db.authenticate_user(user_id).await.map_err(|e| {
        log::error!("Error authenticating user: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    if secret_code_from_db == secret_code {
        log::info!("User authenticated");
        Ok(warp::reply::with_status(
            warp::reply(),
            warp::http::StatusCode::OK,
        ))
    } else {
        log::info!("User not authenticated");
        Err(warp::reject::custom(InvalidToken))
    }
}
