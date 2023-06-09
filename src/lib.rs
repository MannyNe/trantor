pub mod admin;
pub mod db;
pub mod errors;
pub mod middleware;
pub mod session;
pub mod utils;

pub use sqlx;

use std::path::Path;

use db::DB;
use include_dir::{include_dir, Dir};
use sqlx::PgPool;
use uaparser::UserAgentParser;
use warp::Filter;

const REGEXES: &[u8; 205550] = include_bytes!("../data/ua-regexes.yml");
const LAUNCH_CONTROL_JS: &str = include_str!("../data/launch-control.js");

pub async fn server(
    pool: PgPool,
) -> Result<
    impl Filter<Extract = (impl warp::Reply + Send,), Error = warp::Rejection> + Clone,
    sqlx::Error,
> {
    sqlx::migrate!().run(&pool).await?;

    let db = DB::new(pool);
    let ua_parser = UserAgentParser::from_bytes(REGEXES).expect("Failed to make user agent parser");

    let admin_routes = admin::make_admin_routes(db.clone());
    let session_routes = session::make_session_routes(db, ua_parser);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allow_headers(vec![
            "Origin",
            "Content-Type",
            "x-tracking-id",
            "x-source-name",
            "Authorization",
            "Content-Length",
            "Access-Control-Allow-Origin",
        ])
        .allow_credentials(true);

    let fronted_routes = warp::path::tail().and_then(send_file_from_embedded_dir);
    let index_page = warp::get().and_then(index_from_embedded_dir);

    let launch_control_script = warp::path!("launch-control.js")
        .map(|| warp::reply::with_header(LAUNCH_CONTROL_JS, "content-type", "text/javascript"));

    let routes = admin_routes
        .or(session_routes)
        .or(launch_control_script)
        .or(fronted_routes)
        .or(index_page)
        .recover(errors::handle_rejection)
        .with(cors);

    Ok(routes)
}

static FRONTEND_BUILD_DIR: Dir = include_dir!("client/build");

async fn send_file_from_embedded_dir(
    path: warp::path::Tail,
) -> Result<impl warp::Reply, warp::Rejection> {
    let path = Path::new(path.as_str());
    let file = FRONTEND_BUILD_DIR
        .get_file(path)
        .ok_or_else(warp::reject::not_found)?;

    let content_type = match file.path().extension() {
        Some(ext) if ext == "html" => "text/html",
        Some(ext) if ext == "js" => "text/javascript",
        Some(ext) if ext == "css" => "text/css",
        Some(ext) if ext == "png" => "image/png",
        Some(ext) if ext == "svg" => "image/svg+xml",
        Some(ext) if ext == "json" => "application/json",
        Some(ext) if ext == "txt" => "text/plain",
        _ => "application/octet-stream",
    };

    Ok(warp::reply::with_header(
        file.contents(),
        "content-type",
        content_type,
    ))
}

async fn index_from_embedded_dir() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::html(
        FRONTEND_BUILD_DIR
            .get_file("index.html")
            .unwrap()
            .contents(),
    ))
}
