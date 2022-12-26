use crate::structure::structure::DevBoardErrorType;
use crate::structure::structure::DevBoardGenericError;
use crate::structure::structure::SwapRequest;
use crate::util::util_authentication::extract_user_id;
use crate::DB_POOL;
use base64::decode;
use chrono::Utc;
use entity::db_item;
use entity::db_message;
use log::debug;
use sea_orm::prelude::Uuid;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::FromQueryResult;
use sea_orm::IntoActiveModel;
use sea_orm::ModelTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::QuerySelect;

pub async fn get_by_id(id: i32) -> Result<db_item::Model, DevBoardGenericError> {
    let db = DB_POOL.get().await;
    let result = db_item::Entity::find_by_id(id).one(db).await;

    if result.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let opt = result.unwrap();

    if opt.is_none() {
        return Err(DevBoardGenericError {
            success: false,
            code: 2,
            err_type: DevBoardErrorType::Warning,
            message: format!("Item not found"),
        });
    }

    Ok(opt.unwrap())
}

pub async fn get_all() -> Result<Vec<db_item::Model>, DevBoardGenericError> {
    let db = DB_POOL.get().await;

    let result = db_item::Entity::find()
        .order_by_asc(db_item::Column::Order)
        .all(db)
        .await;

    if result.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let models = result.unwrap();
    Ok(models)
}

pub async fn get_by_parent_id(parent_id: i32) -> Result<Vec<db_item::Model>, DevBoardGenericError> {
    let db = DB_POOL.get().await;

    let result = db_item::Entity::find()
        .filter(db_item::Column::ColumnId.eq(parent_id))
        .order_by_asc(db_item::Column::Order)
        .all(db)
        .await;

    if result.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let models = result.unwrap();

    Ok(models)
}

pub struct File {
    file: String,
    file_name: String,
}

pub async fn create(
    json_data: serde_json::Value,
    jwt_opt: Option<String>,
) -> Result<db_item::Model, DevBoardGenericError> {
    let db = DB_POOL.get().await;

    let user_id = extract_user_id(jwt_opt.clone());

    if user_id.is_none() {
        return Err(DevBoardGenericError {
            success: false,
            code: 2,
            err_type: DevBoardErrorType::Warning,
            message: format!("Item not found"),
        });
    }

    save_files(json_data["files"].clone()).await;

    let user_id = user_id.unwrap();

    let result = db_item::ActiveModel::from_json(json_data);

    if result.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let next_order_number = get_next_order_number().await;

    if next_order_number.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error count2(): {:?}", next_order_number.err()),
        });
    }

    let count = next_order_number.unwrap();

    let mut model = result.unwrap();

    let dat = Utc::now().naive_utc();

    model.created_at = sea_orm::Set(Some(dat));
    model.updated_at = sea_orm::Set(Some(dat));
    model.publisher_id = sea_orm::Set(Some(user_id));
    model.order = sea_orm::Set(count);

    let result = model.insert(db).await;

    if result.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    Ok(result.unwrap())
}

async fn save_files(files: serde_json::Value) -> () {
    println!("FILES: {:?}", files);
    let files = files.as_array();
    let files = files.unwrap();
    for file in files {
        let content = file["content"].as_str().unwrap();
        println!("CONTENT: {:?}", content);
        let start_pos = content.chars().position(|c| c == ',').unwrap() + 1;
        let end_pos = content.chars().count();
        println!("TEST: {:?} {:?}", start_pos, end_pos);
        let decoded = &decode(&content[start_pos..end_pos]);
        println!("DECODED: {:?}", decoded);
        let decoded = decoded.clone().unwrap();
        println!("File name: {:?}", file["name"].as_str().unwrap());
        // println!("Content: {:?}", file["content"].as_str().unwrap());
        // println!("File content: {:?}", decoded);

        let file_name = format!(
            "/Users/andrei/Desktop/{}.{}",
            Uuid::new_v4().to_string(),
            "png"
        );
        println!("NEW FILENAME: {:?}", file_name);
        tokio::fs::write(&file_name, decoded)
            .await
            .map_err(|e| {
                eprint!("error writing file: {}", e);
                warp::reject::reject()
            })
            .unwrap();
    }
}

// #[derive(FromQueryResult, Debug)]
// struct CountResult {
//     max: Option<i32>,
// }

// pub async fn get_max_id() -> Result<i32, DevBoardGenericError> {
//     let db = DB_POOL.get().await;

//     let result = db_item::Entity::find()
//         .select_only()
//         .column_as(db_item::Column::Id.max(), "max")
//         .into_model::<CountResult>()
//         .one(db)
//         .await;

//     if result.is_err() {
//         return Err(DevBoardGenericError {
//             success: false,
//             code: 1,
//             err_type: DevBoardErrorType::Error,
//             message: format!("DB Error: {:?}", result.err()),
//         });
//     }
//     let count = result.unwrap().unwrap();

//     if count.max.is_none() {
//         return Ok(0);
//     }

//     Ok(count.max.unwrap())
// }

pub async fn update(
    id: i32,
    json_data: serde_json::Value,
) -> Result<db_item::Model, DevBoardGenericError> {
    let db = DB_POOL.get().await;
    let result = db_item::Entity::find_by_id(id).one(db).await;

    if result.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let opt = result.unwrap();

    if opt.is_none() {
        return Err(DevBoardGenericError {
            success: false,
            code: 2,
            err_type: DevBoardErrorType::Warning,
            message: format!("Item not found"),
        });
    }

    let mut item_active_model: db_item::ActiveModel = opt.unwrap().into();

    let result = item_active_model.set_from_json(json_data);

    if result.is_err() {
        if result.is_err() {
            return Err(DevBoardGenericError {
                success: false,
                code: 1,
                err_type: DevBoardErrorType::Error,
                message: format!("DB Error: {:?}", result.err()),
            });
        }
    }

    let dat = Utc::now().naive_utc();
    item_active_model.updated_at = sea_orm::Set(Some(dat));

    let result = item_active_model.update(db).await;

    if result.is_err() {
        if result.is_err() {
            return Err(DevBoardGenericError {
                success: false,
                code: 1,
                err_type: DevBoardErrorType::Error,
                message: format!("DB Error: {:?}", result.err()),
            });
        }
    }

    Ok(result.unwrap())
}

pub async fn delete(id: i32, jwt_opt: Option<String>) -> Result<bool, DevBoardGenericError> {
    let db = DB_POOL.get().await;

    let user_id = extract_user_id(jwt_opt.clone());

    if user_id.is_none() {
        return Err(DevBoardGenericError {
            success: false,
            code: 2,
            err_type: DevBoardErrorType::Warning,
            message: format!("Item not found"),
        });
    }

    let user_id = user_id.unwrap();

    let messages = db_message::Entity::find()
        .filter(db_message::Column::ItemId.eq(id))
        .filter(db_message::Column::UserId.eq(user_id))
        .all(db)
        .await;

    if messages.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", messages.err()),
        });
    }

    let messages = messages.unwrap();

    for message in messages {
        let result = message.delete(db).await;
        if result.is_err() {
            return Err(DevBoardGenericError {
                success: false,
                code: 1,
                err_type: DevBoardErrorType::Error,
                message: format!("DB Error: {:?}", result.err()),
            });
        }
    }

    let result = db_item::Entity::find_by_id(id)
        .one(db)
        // TODO filter by board owners
        .await;

    if result.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let opt = result.unwrap();

    if opt.is_none() {
        return Err(DevBoardGenericError {
            success: false,
            code: 2,
            err_type: DevBoardErrorType::Warning,
            message: format!("Item not found"),
        });
    }

    let result = opt.unwrap().delete(db).await;

    if result.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }

    let deletion_result = result.unwrap();
    assert_eq!(deletion_result.rows_affected, 1);
    Ok(deletion_result.rows_affected == 1)
}

pub async fn swap(swap_request: SwapRequest) -> Result<bool, DevBoardGenericError> {
    let db = DB_POOL.get().await;
    debug!("IN: {}:{} ", swap_request.id_a, swap_request.id_b);
    let result_a = db_item::Entity::find_by_id(swap_request.id_a).one(db).await;
    let result_b = db_item::Entity::find_by_id(swap_request.id_b).one(db).await;

    if result_a.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result_a.err()),
        });
    }

    if result_b.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result_b.err()),
        });
    }

    let opt_a = result_a.unwrap();
    if opt_a.is_none() {
        return Err(DevBoardGenericError {
            success: false,
            code: 2,
            err_type: DevBoardErrorType::Warning,
            message: format!("Item not found"),
        });
    }

    let opt_b = result_b.unwrap();
    if opt_b.is_none() {
        return Err(DevBoardGenericError {
            success: false,
            code: 2,
            err_type: DevBoardErrorType::Warning,
            message: format!("Item not found"),
        });
    }

    let v_a = opt_a.unwrap();
    let v_b = opt_b.unwrap();

    let order_a = v_a.order;
    let order_b = v_b.order;

    debug!("WHILE: {}:{}", v_a.order, v_b.order);

    let mut active_model_a = v_a.into_active_model();
    let mut active_model_b = v_b.into_active_model();
    active_model_a.order = sea_orm::Set(order_b);
    active_model_b.order = sea_orm::Set(order_a);
    let result_a = active_model_a.update(db).await;
    if result_a.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result_a.err()),
        });
    }
    let result_b = active_model_b.update(db).await;
    if result_b.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result_b.err()),
        });
    }
    Ok(true)
}

#[derive(FromQueryResult, Debug)]
struct OptionResult {
    value: Option<i64>,
}

pub async fn get_next_order_number() -> Result<i64, DevBoardGenericError> {
    let db = DB_POOL.get().await;

    let result = db_item::Entity::find()
        .select_only()
        .column_as(db_item::Column::Order.max(), "value")
        .into_model::<OptionResult>()
        .one(db)
        .await;

    if result.is_err() {
        return Err(DevBoardGenericError {
            success: false,
            code: 1,
            err_type: DevBoardErrorType::Error,
            message: format!("DB Error: {:?}", result.err()),
        });
    }
    let count = result.unwrap().unwrap();

    if count.value.is_none() {
        return Ok(0);
    }

    Ok(count.value.unwrap() + 1)
}
