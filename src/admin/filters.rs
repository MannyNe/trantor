use warp::Filter;

use super::{handlers, CreateSourceRequest};
use crate::db::{with_db, DB};

pub fn make_admin_routes(
    db: DB,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let count_visitors = warp::path!("visitors" / "count")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(handlers::count_visitors);

    let list_visitors = warp::path!("visitors")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(handlers::list_visitors);

    let create_source = warp::path!("sources")
        .and(warp::post())
        .and(warp::body::json::<CreateSourceRequest>())
        .and(with_db(db.clone()))
        .and_then(handlers::create_source);

    let list_sources = warp::path!("sources")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::list_sources);

    warp::path("admin").and(
        count_visitors
            .or(list_visitors)
            .or(create_source)
            .or(list_sources),
    )
}
