use std::{net::SocketAddr, sync::Arc};

use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
use serde::Deserialize;
use sqlx::PgPool;
use tokio::fs;
use trantor::server;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args: Vec<String> = std::env::args().collect();
    let config_path = &args.get(1).ok_or_else(|| eyre!("Missing config path"))?;
    let config = fs::read_to_string(config_path).await?;
    let config: Config = toml::from_str(&config)?;

    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();

    let pool = PgPool::connect(&config.database)
        .await
        .wrap_err_with(|| format!("couldn't connect to database with url: {}", config.database))?;

    let maxmind_reader =
        maxminddb::Reader::open_readfile(&config.geolite2_city).wrap_err_with(|| {
            format!(
                "couldn't open geolite2 city database file: {}",
                config.geolite2_city
            )
        })?;
    let maxmind_reader = Arc::new(maxmind_reader);

    let routes = server(pool, maxmind_reader).await?;
    let addr: SocketAddr = config.address.parse()?;

    if let Some(https) = config.https {
        let key = fs::read(&https.key_path).await.wrap_err_with(|| {
            format!(
                "couldn't read https certificate key from file: {}",
                https.key_path
            )
        })?;
        let cert = fs::read(&https.cert_path).await.wrap_err_with(|| {
            format!(
                "couldn't read https certificate from file: {}",
                https.cert_path
            )
        })?;

        warp::serve(routes)
            .tls()
            .key(key)
            .cert(cert)
            .run(addr)
            .await;
    } else {
        warp::serve(routes).run(addr).await;
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct Config {
    address: String,
    database: String,
    geolite2_city: String,
    https: Option<Https>,
}

#[derive(Debug, Deserialize)]
struct Https {
    cert_path: String,
    key_path: String,
}
