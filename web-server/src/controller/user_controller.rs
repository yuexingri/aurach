use rocket_contrib::json::Json;
use crate::configuration::response::{ApiError, ApiResponse};
use crate::db::mysql_db_pool;
use crate::model::user_model::{User, UserVo};
use crate::service::user_service::{create_user_to_db, get_user_by_id, update_user_by_id};

#[get("/<id>")]
pub fn get_by_id(id: i32, conn: mysql_db_pool::Connection) -> Result<ApiResponse, ApiError> {
    return get_user_by_id(id, &conn);
}

#[put("/<id>/<name>")]
pub fn update_by_id(id: i32, name: String, conn: mysql_db_pool::Connection) -> Result<ApiResponse, ApiError> {
    return update_user_by_id(id, name, &conn);
}

#[post("/", format="json", data="<user>")]
pub fn create_user(user: Json<UserVo>, conn: mysql_db_pool::Connection) -> Result<ApiResponse, ApiError> {
    let user_vo = user.into_inner();
    return create_user_to_db(user_vo.name, &conn);
}
