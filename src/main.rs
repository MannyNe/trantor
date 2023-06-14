use std::env;

use sqlx::PgPool;
use trantor::server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();

    let db_url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&db_url).await?;
    tracing::info!("Connected to {}", &db_url);

    let routes = server(pool).await?;
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
