use warp::Filter;

use super::{handlers, CreateSourceRequest, CreateUserRequest};
use crate::db::{with_db, DB};

fn extract_basic_token() -> impl warp::Filter<Extract = (String,), Error = warp::Rejection> + Clone
{
    warp::any()
        .and(warp::header("Authorization"))
        .and_then(handlers::strip_basic_auth)
}

fn authenticate_filter(db: DB) -> impl Filter<Extract = (), Error = warp::Rejection> + Clone {
    warp::any()
        .and(with_db(db))
        .and(extract_basic_token())
        .and_then(handlers::authenticate_middleware)
        .untuple_one()
}

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
        .and(authenticate_filter(db.clone()))
        .and_then(handlers::count_visitors);

    let list_visitors = warp::path!("visitors")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(authenticate_filter(db.clone()))
        .and_then(handlers::list_visitors);

    let create_source = warp::path!("sources")
        .and(warp::post())
        .and(warp::body::json::<CreateSourceRequest>())
        .and(with_db(db.clone()))
        .and(authenticate_filter(db.clone()))
        .and_then(handlers::create_source);

    let list_sources = warp::path!("sources")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(authenticate_filter(db.clone()))
        .and_then(handlers::list_sources);

    let list_sessions = warp::path!("sessions")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(authenticate_filter(db.clone()))
        .and_then(handlers::list_sessions);

    let tracking_stats_route = warp::path!("tracking" / "stats")
        .and(with_db(db.clone()))
        .and(authenticate_filter(db.clone()))
        .and_then(handlers::tracking_stats);

    let create_user = warp::path!("users")
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(warp::body::json::<CreateUserRequest>())
        .and(authenticate_filter(db))
        .and_then(handlers::create_user);

    warp::path("admin").and(
        count_visitors
            .or(list_visitors)
            .or(create_source)
            .or(list_sources)
            .or(list_sessions)
            .or(tracking_stats_route)
            .or(create_user)
            .or(authenticate_user),
    )
}
