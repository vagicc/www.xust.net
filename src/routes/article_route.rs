use crate::handlers::article_handler;
use warp::Filter;

pub fn list() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let first = warp::get()
        .and(warp::path("article"))
        .and(warp::path("index"))
        .and(warp::path::end())
        .and_then(|| async { article_handler::list(1).await });

    warp::get()
        .and(warp::path("article"))
        .and(warp::path("index"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(article_handler::list)
        .or(first)
        .or(detail())
}

// /article/detail/42
pub fn detail() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("article"))
        .and(warp::path("detail"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(article_handler::detail)
}
