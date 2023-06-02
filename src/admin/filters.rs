use warp::Filter;

use super::{handlers, CreateSourceRequest, CreateTrackingRequest, CreateUserRequest};
use crate::{
    db::{with_db, DB},
    middleware::{authenticate_filter, authenticate_filter_extract_user_id},
};

pub fn make_admin_routes(
    db: DB,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let authenticate_user = warp::path!("authenticate")
        .and(warp::post())
        .and(authenticate_filter(db.clone()))
        .and_then(handlers::authenticate_user);

    let count_visitors = warp::path!("visitors" / "count")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(authenticate_filter_extract_user_id(db.clone()))
        .and_then(handlers::count_visitors);

    let create_source = warp::path!("sources")
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(warp::body::json::<CreateSourceRequest>())
        .and(authenticate_filter_extract_user_id(db.clone()))
        .and_then(handlers::create_source);

    let list_sources = warp::path!("sources")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(authenticate_filter_extract_user_id(db.clone()))
        .and_then(handlers::list_sources);

    let list_sessions = warp::path!("sessions")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(authenticate_filter_extract_user_id(db.clone()))
        .and_then(handlers::list_sessions);

    let list_trackings = warp::path!("trackings")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(authenticate_filter_extract_user_id(db.clone()))
        .and_then(handlers::list_trackings);

    let create_tracking = warp::path!("trackings")
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(authenticate_filter_extract_user_id(db.clone()))
        .and(warp::body::json::<CreateTrackingRequest>())
        .and_then(handlers::create_tracking);

    let get_tracking = warp::path!("trackings" / String)
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(authenticate_filter_extract_user_id(db.clone()))
        .and_then(handlers::get_tracking);

    let list_visitors = warp::path!("trackings" / String / "visitors")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(authenticate_filter_extract_user_id(db.clone()))
        .and_then(handlers::list_visitors);

    let create_user = warp::path!("users")
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(warp::body::json::<CreateUserRequest>())
        .and_then(handlers::create_user);

    warp::path("admin").and(
        count_visitors
            .or(list_visitors)
            .or(create_source)
            .or(list_sources)
            .or(list_sessions)
            .or(list_trackings)
            .or(create_tracking)
            .or(get_tracking)
            .or(create_user)
            .or(authenticate_user),
    )
}
