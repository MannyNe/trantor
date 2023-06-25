use color_eyre::{
    eyre::{eyre, Context},
    Result,
};

use controllers::{warp, Controllers};
use pg_repositories::{sqlx::PgPool, PgSessionsRepository, PgVisitorsRepository};
use services::SessionStartService;

mod ua_parser;
use ua_parser::UAParser;

mod maxmind_reader;
use maxmind_reader::MaxmindGeoIpReader;

mod config;
use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let config = make_config()?;

    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();

    let pool = PgPool::connect(config.database_url())
        .await
        .wrap_err_with(|| {
            format!(
                "couldn't connect to database with url: {}",
                config.database_url()
            )
        })?;

    let user_agent_parser = UAParser::new();
    let sessions = PgSessionsRepository::new(pool.clone());
    let visitors = PgVisitorsRepository::new(pool);
    let geo_ip_reader = MaxmindGeoIpReader::new(config.maxminddb_path()).wrap_err_with(|| {
        format!(
            "couldn't open geolite2 city database file: {}",
            config.maxminddb_path()
        )
    })?;

    let session_start_service =
        SessionStartService::new(sessions, visitors, user_agent_parser, geo_ip_reader);

    let controllers = Controllers::new(session_start_service);
    let routes = controllers.routes();

    warp::serve(routes).run(config.address()).await;

    Ok(())
}

fn make_config() -> Result<Config> {
    let args: Vec<String> = std::env::args().collect();
    let config_path = &args.get(1).ok_or_else(|| eyre!("Missing config path"))?;
    let config = Config::from_file(&config_path)?;
    Ok(config)
}
