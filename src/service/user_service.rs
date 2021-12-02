use crate::model::user_model::{User, get_user_by_id_from_db};

pub fn get_user_by_id(id: u8) -> User {
    return get_user_by_id_from_db(id);
}