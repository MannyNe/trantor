use std::convert::Infallible;

use serde::Serialize;
use warp::{hyper::StatusCode, reject, Rejection, Reply};

#[derive(Debug)]
pub struct DatabaseError;
impl reject::Reject for DatabaseError {}

#[derive(Debug)]
pub struct MissingSessionId;
impl reject::Reject for MissingSessionId {}

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(DatabaseError) = err.find() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "DATABASE_ERROR";
    } else if let Some(MissingSessionId) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "MISSING_SESSION_ID";
    } else if err
        .find::<warp::filters::body::BodyDeserializeError>()
        .is_some()
    {
        // This error happens if the body could not be deserialized correctly
        // We can use the cause to analyze the error and customize the error message
        code = StatusCode::BAD_REQUEST;
        message = "Failed to deserialize body";
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
