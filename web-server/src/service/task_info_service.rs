use rocket_contrib::json;
use crate::configuration::response::{ApiError, ApiResponse, db_error, success};
use crate::db::mysql_db_pool;
use crate::model::task_info_model::TaskInfo;

pub fn get_task_info_by_id(id: i32, conn: &mysql_db_pool::Connection) -> Result<ApiResponse, ApiError> {
    let result = TaskInfo::read_by_id(id, conn);
    match result {
        Ok(r) => Ok(success(json!({
            "id": r.id,
            "name": r.name,
        }))),
        Err(e) => Err(db_error(e)),
    }
}