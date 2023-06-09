use std::env;

use sqlx::PgPool;
use trantor::server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let db_url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&db_url).await?;
    log::info!("Connected to {}", &db_url);

    let routes = server(pool).await?;
    warp::serve(routes)
        .tls()
        .cert_path("/etc/letsencrypt/live/trantor.frectonz.tech/fullchain.pem")
        .key_path("/etc/letsencrypt/live/trantor.frectonz.tech/privkey.pem")
        .run(([0, 0, 0, 0], 443))
        .await;

    Ok(())
}
