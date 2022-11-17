use crate::Structs::DaoError;
use crate::DB_POOL;
use chrono::Utc;
use entity::db_item;
use entity::db_user;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::FromQueryResult;
use sea_orm::ModelTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;

pub async fn get_by_id(id: i32) -> Result<db_user::Model, DaoError> {
    let db = DB_POOL.get().await;
    let result = db_user::Entity::find_by_id(id).one(db).await;

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let opt = result.unwrap();

    if opt.is_none() {
        return Err(DaoError {
            code: 2,
            err_type: crate::Structs::DaoErrorType::Warning,
            message: format!("Item not found"),
        });
    }

    Ok(opt.unwrap())
}

pub async fn get_by_name(username: String) -> Result<Option<db_user::Model>, DaoError> {
    let db = DB_POOL.get().await;
    let result = db_user::Entity::find()
        .filter(db_user::Column::Username.eq(username))
        .one(db)
        .await;

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let opt = result.unwrap();

    Ok(opt)
}

pub async fn get_all() -> Result<Vec<db_user::Model>, DaoError> {
    let db = DB_POOL.get().await;

    let result = db_user::Entity::find()
        .order_by_asc(db_user::Column::Id)
        .all(db)
        .await;

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let models = result.unwrap();

    Ok(models)
}

pub async fn create(json_data: serde_json::Value) -> Result<db_user::Model, DaoError> {
    let db = DB_POOL.get().await;
    let result = db_user::ActiveModel::from_json(json_data);

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let mut model = result.unwrap();

    let dat = Utc::now().naive_utc();
    model.created_at = sea_orm::Set(Some(dat));
    model.updated_at = sea_orm::Set(Some(dat));

    let result = model.insert(db).await;

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    Ok(result.unwrap())
}

pub async fn update(id: i32, json_data: serde_json::Value) -> Result<db_user::Model, DaoError> {
    let db = DB_POOL.get().await;
    let result = db_user::Entity::find_by_id(id).one(db).await;

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let opt = result.unwrap();

    if opt.is_none() {
        return Err(DaoError {
            code: 2,
            err_type: crate::Structs::DaoErrorType::Warning,
            message: format!("Item not found"),
        });
    }

    let mut item_active_model: db_user::ActiveModel = opt.unwrap().into();

    let result = item_active_model.set_from_json(json_data);

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let dat = Utc::now().naive_utc();
    item_active_model.updated_at = sea_orm::Set(Some(dat));

    let result = item_active_model.update(db).await;

    if result.is_err() {
        if result.is_err() {
            return Err(DaoError {
                code: 1,
                err_type: crate::Structs::DaoErrorType::Error,
                message: format!("DB Error: {:?}", result.err()),
            });
        }
    }

    Ok(result.unwrap())
}

pub async fn delete(id: i32) -> Result<bool, DaoError> {
    let db = DB_POOL.get().await;

    let result = db_user::Entity::find_by_id(id).one(db).await;

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let opt = result.unwrap();

    if opt.is_none() {
        return Err(DaoError {
            code: 2,
            err_type: crate::Structs::DaoErrorType::Warning,
            message: format!("Item not found"),
        });
    }

    let items_result = db_item::Entity::find()
        .filter(db_item::Column::ColumnId.eq(id))
        .all(db)
        .await;

    if items_result.is_err() {
        return Err(DaoError {
            code: 2,
            err_type: crate::Structs::DaoErrorType::Warning,
            message: format!("Error while retrieving items"),
        });
    }

    let items = items_result.unwrap();

    for item in items.into_iter() {
        let item_result = item.delete(db).await;
        if item_result.is_err() {
            return Err(DaoError {
                code: 2,
                err_type: crate::Structs::DaoErrorType::Warning,
                message: format!("Error while deleting item"),
            });
        }
    }

    let result = opt.unwrap().delete(db).await;

    if result.is_err() {
        return Err(DaoError {
            code: 1,
            err_type: crate::Structs::DaoErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let deletion_result = result.unwrap();
    assert_eq!(deletion_result.rows_affected, 1);
    Ok(deletion_result.rows_affected == 1)
}
