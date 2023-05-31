use std::sync::Arc;

use tokio::fs;
use warp::Filter;

use super::{handlers, CreateSourceRequest, CreateUserRequest};
use crate::{
    db::{with_db, DB},
    errors::InvalidToken,
};

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
        .and(with_db(db.clone()))
        .and_then(handlers::list_sources);

    let list_sessions = warp::path!("sessions")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(handlers::list_sessions);

    let liquid_parser = liquid::ParserBuilder::with_stdlib()
        .build()
        .expect("Failed to build liquid parser");
    let liquid_parser = Arc::new(liquid_parser);

    fn with_template(
        template_file: &'static str,
        liquid_parser: Arc<liquid::Parser>,
    ) -> impl Filter<Extract = (liquid::Template,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || liquid_parser.clone()).then(
            move |liquid_parser: Arc<liquid::Parser>| async move {
                let home_page = fs::read_to_string(template_file)
                    .await
                    .expect("Failed to read template file");

                let home_page_template = liquid_parser
                    .parse(&home_page)
                    .expect("Failed to parse template");

                log::info!("Loaded template from {}", template_file);

                home_page_template
            },
        )
    }

    let admin_home_page = warp::path::end()
        .and(with_db(db.clone()))
        .and(with_template("templates/home.html", liquid_parser))
        .and_then(handlers::home_page);

    let create_user = warp::path!("users")
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(warp::body::json::<CreateUserRequest>())
        .and_then(handlers::create_user);

    async fn strip_basic_auth(auth: String) -> Result<String, warp::Rejection> {
        match auth.strip_prefix("Basic ") {
            Some(token) => Ok(token.to_string()),
            None => Err(warp::reject::custom(InvalidToken)),
        }
    }

    fn extract_basic_token() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
        warp::any()
            .and(warp::header("Authorization"))
            .and_then(strip_basic_auth)
    }

    let authenticate_user = warp::path!("authenticate")
        .and(warp::post())
        .and(with_db(db))
        .and(extract_basic_token())
        .and_then(handlers::authenticate_user);

    warp::path("admin").and(
        count_visitors
            .or(list_visitors)
            .or(create_source)
            .or(list_sources)
            .or(list_sessions)
            .or(admin_home_page)
            .or(create_user)
            .or(authenticate_user),
    )
}
