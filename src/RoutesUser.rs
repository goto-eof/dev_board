use warp::{Filter, Rejection, Reply};

use crate::{AuthenticationUtil::auth_validator, ControllerAuth, ControllerUser};

pub async fn get_user_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let db_column = warp::path("user");
    db_column
        .and(warp::path("register"))
        .and(warp::post())
        .and(warp::path::end())
        // .and(auth_validator("register".to_string()).await)
        // .untuple_one()
        .and(warp::body::json())
        .and(warp::body::content_length_limit(1024 * 16))
        .and_then(ControllerAuth::register)
        .or(db_column
            .and(warp::path("login"))
            .and(warp::post())
            .and(warp::path::end())
            // .and(auth_validator("login".to_string()).await)
            // .untuple_one()
            .and(warp::body::json())
            .and(warp::body::content_length_limit(1024 * 16))
            .and_then(ControllerAuth::login))
        .or(db_column
            .and(auth_validator("get_user".to_string()).await)
            .untuple_one()
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and_then(ControllerUser::get_user))
        .or(db_column
            .and(warp::get())
            .and(auth_validator("get_all_users".to_string()).await)
            .untuple_one()
            .and(warp::path::end())
            .and_then(ControllerUser::get_all_users))
        .or(db_column
            .and(warp::post())
            .and(auth_validator("insert_user".to_string()).await)
            .untuple_one()
            .and(warp::path::end())
            .and(warp::body::json())
            .and_then(ControllerUser::insert_user))
        .or(db_column
            .and(warp::put())
            .and(auth_validator("update_user".to_string()).await)
            .untuple_one()
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and(warp::body::json())
            .and_then(ControllerUser::update_user))
        .or(db_column
            .and(warp::delete())
            .and(auth_validator("delete_user".to_string()).await)
            .untuple_one()
            .and(warp::path::param::<i32>())
            .and(warp::path::end())
            .and_then(ControllerUser::delete_user))
}