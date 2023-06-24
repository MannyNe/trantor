mod session_start;

pub use session_start::*;

use domain::Service;
use warp::Filter;
pub fn warp_service<S: Service + Clone + Send + Sync>(
    service: S,
) -> impl warp::Filter<Extract = (S,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || service.clone())
}
