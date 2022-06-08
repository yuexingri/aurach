use rocket_contrib::json;

use crate::configuration::response::{db_error, success, ApiError, ApiResponse};
use crate::db::mysql_db_pool;
use crate::model::user_model::User;

pub fn get_user_by_id(id: i32, conn: &mysql_db_pool::Connection) -> Result<ApiResponse, ApiError> {
    let result = User::read_by_id(id, conn);
    match result {
        Ok(r) => Ok(success(json!(r))),
        Err(e) => Err(db_error(e)),
    }
}

pub fn update_user_by_id(id: i32, name: String, conn: &mysql_db_pool::Connection) -> Result<ApiResponse, ApiError> {
    let result = User::update(id, User { id: Option::Some(id), name: Option::Some(name) }, conn);
    match result {
        Ok(r) => Ok(success(json!(r))),
        Err(e) => Err(db_error(e)),
    }
}

pub fn create_user_to_db(user: User, conn: &mysql_db_pool::Connection) -> Result<ApiResponse, ApiError> {
    let result  = User::create(user, conn);
    match result {
        Ok(r) => Ok(success(json!(r))),
        Err(e) => Err(db_error(e)),
    }
}


