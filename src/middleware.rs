use warp::Filter;

use crate::{
    db::DB,
    errors::{DatabaseError, InvalidBase64, InvalidToken},
};

pub fn extract_basic_token(
) -> impl warp::Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::header("Authorization"))
        .and_then(strip_basic_auth)
}

async fn strip_basic_auth(auth: String) -> Result<String, warp::Rejection> {
    auth.strip_prefix("Basic ").map_or_else(
        || Err(warp::reject::custom(InvalidToken)),
        |token| Ok(token.to_string()),
    )
}

pub async fn authenticate_filter(db: DB, token: String) -> Result<(DB, i32), warp::Rejection> {
    tracing::info!("Authenticating user with token: {}", token);

    let engine = base64::engine::general_purpose::URL_SAFE;

    use base64::Engine;

    let decoded = engine.decode(token).map_err(|e| {
        tracing::error!("Error decoding token: {}", e);
        warp::reject::custom(InvalidBase64)
    })?;
    let decode = String::from_utf8(decoded).map_err(|e| {
        tracing::error!("Error decoding token to UTF-8: {}", e);
        warp::reject::custom(InvalidBase64)
    })?;

    let (user_id, secret_code) = decode.split_once(':').ok_or_else(|| {
        tracing::error!("Error splitting token into user ID and secret code");
        warp::reject::custom(InvalidBase64)
    })?;

    let (user_id, secret_code_from_db) = db.authenticate_user(user_id).await.map_err(|e| {
        tracing::error!("Error authenticating user: {}", e);
        warp::reject::custom(DatabaseError)
    })?;

    if secret_code_from_db == secret_code {
        tracing::info!("User authenticated");
        Ok((db, user_id))
    } else {
        tracing::info!("User not authenticated");
        Err(warp::reject::custom(InvalidToken))
    }
}

pub async fn user_id_owns_tracking(
    (db, user_id): (DB, i32),
    tracking_id: String,
) -> Result<(DB, i32), warp::Rejection> {
    let (tracking_id, owner_id) = db
        .tracking_owner_and_primary_key(&tracking_id)
        .await
        .map_err(|e| {
            tracing::error!("Error getting owner id: {}", e);
            warp::reject::custom(DatabaseError)
        })?;

    if owner_id != user_id {
        tracing::error!("User {} tried to access tracking {}", user_id, tracking_id);
        return Err(warp::reject::custom(DatabaseError));
    }

    Ok((db, tracking_id))
}
