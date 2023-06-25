use color_eyre::{eyre::Context, Result};

use controllers::{warp, Controllers};
use pg_repositories::{sqlx::PgPool, PgSessionsRepository, PgVisitorsRepository};
use services::SessionStartService;

mod ua_parser;
use ua_parser::UAParser;

mod maxmind_reader;
use maxmind_reader::MaxmindGeoIpReader;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let pool = PgPool::connect("")
        .await
        .wrap_err_with(|| format!("couldn't connect to database with url: {}", ""))?;

    let sessions = PgSessionsRepository::new(pool.clone());
    let visitors = PgVisitorsRepository::new(pool);
    let user_agent_parser = UAParser::new();
    let geo_ip_reader = MaxmindGeoIpReader::new("")
        .wrap_err_with(|| format!("couldn't open geolite2 city database file: {}", ""))?;

    let session_start_service =
        SessionStartService::new(sessions, visitors, user_agent_parser, geo_ip_reader);

    let controllers = Controllers::new(session_start_service);
    let routes = controllers.routes();

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
