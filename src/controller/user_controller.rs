use crate::configuration::response::{ApiError, ApiResponse};
use crate::db::mysql_db_pool;
use crate::service::user_service::get_user_by_id;

#[get("/<id>")]
pub fn get(id: i32, conn: mysql_db_pool::Connection) -> Result<ApiResponse, ApiError> {
    return get_user_by_id(id, &conn);
}
