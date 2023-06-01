use warp::{reject, Filter};

use crate::{
    db::{with_db, DB},
    errors::DatabaseError,
};

async fn extract_tracking_id(db: DB, tracking_id: String) -> Result<i32, reject::Rejection> {
    let tracking_id = db.id_from_tracking_id(&tracking_id).await.map_err(|e| {
        log::error!("Error getting tracking id: {}", e);
        reject::custom(DatabaseError)
    })?;

    Ok(tracking_id)
}

pub fn extract_tracking_id_filter(
    db: DB,
) -> impl Filter<Extract = (i32,), Error = warp::Rejection> + Clone {
    with_db(db)
        .and(warp::header("x-tracking-id"))
        .and_then(extract_tracking_id)
}
