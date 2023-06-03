mod tracking_id;
mod user_id;

pub use tracking_id::*;
pub use user_id::*;

use crate::{db::DB, errors::DatabaseError};

pub async fn user_id_owns_tracking(
    (db, user_id): (DB, i32),
    tracking_id: String,
) -> Result<(DB, i32), warp::Rejection> {
    let (tracking_id, owner_id) = db
        .tracking_owner_and_primary_key(&tracking_id)
        .await
        .map_err(|e| {
            log::error!("Error getting owner id: {}", e);
            warp::reject::custom(DatabaseError)
        })?;

    if owner_id != user_id {
        log::error!("User {} tried to access tracking {}", user_id, tracking_id);
        return Err(warp::reject::custom(DatabaseError));
    }

    Ok((db, tracking_id))
}
