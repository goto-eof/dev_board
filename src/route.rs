use mobc::Pool;
use mobc_postgres::{tokio_postgres::NoTls, PgConnectionManager};
use warp::{Filter, Rejection, Reply};

use crate::{handler, with_db};

pub fn get_routes(
    db_pool: Pool<PgConnectionManager<NoTls>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let todo = warp::path("column");
    todo.and(warp::get())
        .and(warp::query())
        .and(with_db(db_pool.clone()))
        .and_then(handler::list_column_items_handler)
        .or(todo
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::create_column_items_handler))
        .or(todo
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::update_column_items_handler))
        .or(todo
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db_pool.clone()))
            .and_then(handler::delete_column_items_handler))
}