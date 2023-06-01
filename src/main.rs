use std::env;

use color_eyre::Result;
use sqlx::PgPool;
use uaparser::UserAgentParser;
use warp::Filter;

use trantor::{admin, db::DB, errors, session};

const REGEXES: &[u8; 205550] = include_bytes!("../data/ua-regexes.yml");
const PORT: u16 = 3030;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let db_url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&db_url).await?;
    log::info!("Connected to {}", &db_url);

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
            "Authorization",
            "Content-Length",
            "Access-Control-Allow-Origin",
        ])
        .allow_credentials(true);

    let routes = session_routes
        .or(admin_routes)
        .with(cors)
        .recover(errors::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], PORT)).await;

    Ok(())
}
