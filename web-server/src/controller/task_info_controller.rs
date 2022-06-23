use crate::configuration::response::{ApiError, ApiResponse};
use crate::db::mysql_db_pool;
use crate::service::task_info_service::get_task_info_by_id;

#[get("/<id>")]
pub fn get_by_id(id: i32, conn: mysql_db_pool::Connection) -> Result<ApiResponse, ApiError> {
    return get_task_info_by_id(id, &conn);
}