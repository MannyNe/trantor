use warp::Filter;

use super::{handlers, CreateSourceRequest, CreateTrackingRequest, CreateUserRequest};
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

    let count_visitors = warp::path!("visitors" / "count")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(extract_basic_token())
        .and_then(authenticate_filter)
        .map(|(db, _)| db)
        .and_then(handlers::count_visitors);

    let list_sessions = warp::path!("sessions")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(extract_basic_token())
        .and_then(authenticate_filter)
        .map(|(db, _)| db)
        .and_then(handlers::list_sessions);

    let list_trackings = warp::path!("trackings")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(extract_basic_token())
        .and_then(authenticate_filter)
        .and_then(handlers::list_trackings);

    let create_tracking = warp::path!("trackings")
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(extract_basic_token())
        .and_then(authenticate_filter)
        .and(warp::body::json::<CreateTrackingRequest>())
        .and_then(handlers::create_tracking);

    let get_tracking = warp::get()
        .and(with_db(db.clone()))
        .and(extract_basic_token())
        .and_then(authenticate_filter)
        .and(warp::path!("trackings" / String))
        .and_then(user_id_owns_tracking)
        .and_then(|(db, tracking_id)| handlers::get_tracking(db, tracking_id));

    let list_visitors = warp::get()
        .and(with_db(db.clone()))
        .and(extract_basic_token())
        .and_then(authenticate_filter)
        .and(warp::path!("trackings" / String / "visitors"))
        .and_then(user_id_owns_tracking)
        .and_then(|(db, tracking_id)| handlers::list_visitors(db, tracking_id));

    let list_sources = warp::get()
        .and(with_db(db.clone()))
        .and(extract_basic_token())
        .and_then(authenticate_filter)
        .and(warp::path!("trackings" / String / "sources"))
        .and_then(user_id_owns_tracking)
        .and_then(|(db, tracking_id)| handlers::list_sources(db, tracking_id));

    let count_paths = warp::get()
        .and(with_db(db.clone()))
        .and(extract_basic_token())
        .and_then(authenticate_filter)
        .and(warp::path!("trackings" / String / "paths"))
        .and_then(user_id_owns_tracking)
        .and_then(|(db, tracking_id)| handlers::count_paths(db, tracking_id));

    let create_source = warp::post()
        .and(with_db(db.clone()))
        .and(extract_basic_token())
        .and_then(authenticate_filter)
        .and(warp::path!("trackings" / String / "sources"))
        .and_then(user_id_owns_tracking)
        .and(warp::body::json::<CreateSourceRequest>())
        .and_then(|(db, tracking_id), source| handlers::create_source(db, tracking_id, source));

    let create_user = warp::path!("users")
        .and(warp::post())
        .and(with_db(db))
        .and(warp::body::json::<CreateUserRequest>())
        .and_then(handlers::create_user);

    warp::path("admin").and(
        count_visitors
            .or(list_visitors)
            .or(create_source)
            .or(list_sources)
            .or(list_sessions)
            .or(list_trackings)
            .or(count_paths)
            .or(create_tracking)
            .or(get_tracking)
            .or(create_user)
            .or(authenticate_user),
    )
}
