use diesel::result::Error;
use diesel::{MysqlConnection, QueryDsl, RunQueryDsl};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use chrono::NaiveDateTime;
use serde::{Serialize, Serializer};
use crate::model::schema::aurach_task_info;

#[derive(Queryable, PartialEq, Debug)]
pub struct TaskInfo {
    pub id: i32,
    pub name: Option<String>,
    status: i32,
    create_time: NaiveDateTime,
    update_time: NaiveDateTime,
}


impl TaskInfo {
    pub fn read_by_id(id: i32, conn: &MysqlConnection) -> Result<TaskInfo, Error> {
        aurach_task_info::table.find(id).first(conn)
    }
}