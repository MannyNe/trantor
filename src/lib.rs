pub mod admin;
pub mod db;
pub mod errors;
pub mod middleware;
pub mod session;
pub mod utils;

pub use sqlx;

use std::path::Path;

use db::DB;
use include_dir::{include_dir, Dir, File};
use sqlx::{
    types::chrono::{self, Utc},
    PgPool,
};
use uaparser::UserAgentParser;
use warp::{filters::compression, http::Response, path::Tail, Filter};

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

    let fronted_routes = warp::path::tail()
        .and_then(send_file_from_embedded_dir)
        .with(compression::gzip());
    let index_page = warp::get()
        .map(index_from_embedded_dir)
        .with(compression::gzip());

    let launch_control_script = warp::path!("launch-control.js")
        .map(|| {
            let resp = Response::builder()
                .header("content-type", "text/javascript")
                .header("content-length", LAUNCH_CONTROL_JS.len())
                .header("cache-control", "max-age=604800") // 7 days
                .body(LAUNCH_CONTROL_JS)
                .expect("Failed to build response for launch-control.js");

            resp
        })
        .with(compression::gzip());

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

async fn send_file_from_embedded_dir(path: Tail) -> Result<impl warp::Reply, warp::Rejection> {
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

    let last_modified = get_last_modified_date(file);

    let resp = Response::builder()
        .header("content-type", content_type)
        .header("content-length", file.contents().len())
        .header("last-modified", last_modified)
        .header("cache-control", "max-age=31536000") // 1 year
        .body(file.contents())
        .expect("Failed to build response for file");

    Ok(resp)
}

fn index_from_embedded_dir() -> impl warp::Reply {
    let index_html = FRONTEND_BUILD_DIR
        .get_file("index.html")
        .expect("Failed to get index.html");
    let last_modified = get_last_modified_date(index_html);

    Response::builder()
        .header("content-type", "text/html")
        .header("content-length", index_html.contents().len())
        .header("last-modified", last_modified)
        .header("cache-control", "max-age=31536000") // 1 year
        .body(index_html.contents())
        .expect("Failed to build response for index.html")
}

fn get_last_modified_date(file: &File<'_>) -> String {
    let last_modified = file
        .metadata()
        .map(|m| m.modified())
        .unwrap_or_else(std::time::SystemTime::now);
    let last_modified = chrono::DateTime::<Utc>::from(last_modified)
        .format("%a, %d %b %Y %H:%M:%S GMT")
        .to_string();

    last_modified
}
