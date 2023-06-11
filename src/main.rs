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
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
