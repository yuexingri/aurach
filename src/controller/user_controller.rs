use crate::model::user_model::User;
use crate::service::user_service::get_user_by_id;

#[get("/<id>")]
pub fn get(id: u8) -> User {
    return get_user_by_id(id);
}