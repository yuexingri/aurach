use rocket_contrib::json;

use crate::configuration::response::{ApiError, ApiResponse, db_error, success};
use crate::db::mysql_db_pool;
use crate::model::user_model::User;

pub fn get_user_by_id(id: i32, conn: &mysql_db_pool::Connection) -> Result<ApiResponse, ApiError> {
    let result = User::read_by_id(id, conn);
    match result {
        Ok(r) => Ok(success(json!(r))),
        Err(e) => Err(db_error(e)),
    }
}