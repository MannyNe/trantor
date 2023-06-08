use warp::Filter;

use super::{
    handlers, CreateSourceRequest, CreateTrackingRequest, CreateUserRequest, RenameTrackingRequest,
};
use crate::{
    db::{with_db, DB},
    middleware::{authenticate_filter, extract_basic_token, user_id_owns_tracking},
};

pub fn make_admin_routes(
    db: DB,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let authenticate_user = warp::path!("authenticate")
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(extract_basic_token())
        .and_then(authenticate_filter)
        .map(|(_, _)| ())
        .untuple_one()
        .and_then(handlers::authenticate_user);
    let create_user = warp::path!("users")
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(warp::body::json::<CreateUserRequest>())
        .and_then(handlers::create_user);

    let create_tracking = warp::path!("trackings")
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(extract_basic_token())
        .and_then(authenticate_filter)
        .and(warp::body::json::<CreateTrackingRequest>())
        .and_then(handlers::create_tracking);
    let list_trackings = warp::path!("trackings")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(extract_basic_token())
        .and_then(authenticate_filter)
        .and_then(handlers::list_trackings);
    let get_tracking = warp::get()
        .and(with_db(db.clone()))
        .and(extract_basic_token())
        .and_then(authenticate_filter)
        .and(warp::path!("trackings" / String))
        .and_then(user_id_owns_tracking)
        .and_then(|(db, tracking_id)| handlers::get_tracking(db, tracking_id));
    let tracking_counts = warp::get()
        .and(with_db(db.clone()))
        .and(extract_basic_token())
        .and_then(authenticate_filter)
        .and(warp::path!("trackings" / String / "counts"))
        .and_then(user_id_owns_tracking)
        .and_then(|(db, tracking_id)| handlers::tracking_counts(db, tracking_id));
    let patch_tracking_name = warp::patch()
        .and(with_db(db.clone()))
        .and(extract_basic_token())
        .and_then(authenticate_filter)
        .and(warp::path!("trackings" / String / "name"))
        .and_then(user_id_owns_tracking)
        .and(warp::body::json::<RenameTrackingRequest>())
        .and_then(|(db, tracking_id), req| handlers::rename_tracking(db, tracking_id, req));
    let delete_tracking = warp::delete()
        .and(with_db(db.clone()))
        .and(extract_basic_token())
        .and_then(authenticate_filter)
        .and(warp::path!("trackings" / String))
        .and_then(user_id_owns_tracking)
        .and_then(|(db, tracking_id)| handlers::delete_tracking(db, tracking_id));

    let create_source = warp::post()
        .and(with_db(db.clone()))
        .and(extract_basic_token())
        .and_then(authenticate_filter)
        .and(warp::path!("trackings" / String / "sources"))
        .and_then(user_id_owns_tracking)
        .and(warp::body::json::<CreateSourceRequest>())
        .and_then(|(db, tracking_id), source| handlers::create_source(db, tracking_id, source));
    let delete_source = warp::delete()
        .and(with_db(db))
        .and(extract_basic_token())
        .and_then(authenticate_filter)
        .and(warp::path!("trackings" / String / "sources" / String))
        .and_then(|first, tracking_id, source_name| async move {
            user_id_owns_tracking(first, tracking_id)
                .await
                .map(|(db, tracking_id)| (db, tracking_id, source_name))
        })
        .and_then(|(db, tracking_id, source_name)| {
            handlers::delete_source(db, tracking_id, source_name)
        });

    warp::path("admin").and(
        authenticate_user
            .or(create_user)
            .or(create_tracking)
            .or(list_trackings)
            .or(get_tracking)
            .or(tracking_counts)
            .or(patch_tracking_name)
            .or(delete_tracking)
            .or(create_source)
            .or(delete_source),
    )
}
