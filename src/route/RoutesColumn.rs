use warp::{Filter, Rejection, Reply};

use crate::{controller::ControllerColumn, util::AuthenticationUtil::auth_validator};
pub async fn get_column_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let db_column = warp::path("column");
    db_column
        .and(warp::get())
        .and(auth_validator("get_column".to_string()).await)
        .untuple_one()
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and_then(ControllerColumn::get_column)
        .or(db_column
            .and(warp::get())
            .and(auth_validator("get_all_columns".to_string()).await)
            .untuple_one()
            .and(warp::path::end())
            .and_then(ControllerColumn::get_all_columns))
        .or(db_column
            .and(warp::get())
            .and(auth_validator("get_all_columns_with_items".to_string()).await)
            .untuple_one()
            .and(warp::path("plus-items"))
            .and(warp::path::end())
            .and_then(ControllerColumn::get_all_columns_with_items))
        .or(db_column
            .and(warp::post())
            .and(warp::path::end())
            .and(auth_validator("insert_column".to_string()).await)
            .untuple_one()
            .and(warp::body::json())
            .and(warp::body::content_length_limit(1024 * 16))
            .and_then(ControllerColumn::insert_column))
        .or(db_column
            .and(warp::put())
            .and(auth_validator("update_column".to_string()).await)
            .untuple_one()
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(warp::body::json())
            .and(warp::body::content_length_limit(1024 * 16))
            .and_then(ControllerColumn::update_column))
        .or(db_column
            .and(warp::put())
            .and(auth_validator("swap_columns".to_string()).await)
            .untuple_one()
            .and(warp::path("swap"))
            .and(warp::path::end())
            .and(warp::body::json())
            .and(warp::body::content_length_limit(1024 * 16))
            .and_then(ControllerColumn::swap_columns))
        .or(db_column
            .and(warp::delete())
            .and(auth_validator("delete_column".to_string()).await)
            .untuple_one()
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and_then(ControllerColumn::delete_column))
}