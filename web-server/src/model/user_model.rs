use diesel::result::Error;
use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::model::schema::aurach_user;

#[derive(Debug, Queryable, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "aurach_user"]
pub struct User {
    pub id: Option<i32>,
    pub name: Option<String>,
}

impl User {
    pub fn create(user: User, conn: &MysqlConnection) -> Result<User, Error> {
        let user = User { ..user };

        let ops = diesel::insert_into(aurach_user::table)
            .values(&user)
            .execute(conn);

        match ops {
            Ok(_) => aurach_user::table.order(aurach_user::id.desc()).first(conn),
            Err(e) => Err(e),
        }
    }

    pub fn read(conn: &MysqlConnection) -> Result<Vec<User>, Error> {
        aurach_user::table
            .order(aurach_user::id.asc())
            .load::<User>(conn)
    }

    pub fn read_by_id(id: i32, conn: &MysqlConnection) -> Result<User, Error> {
        aurach_user::table.find(id).first(conn)
    }

    pub fn update(id: i32, user: User, conn: &MysqlConnection) -> Result<User, Error> {
        let user = User { ..user };

        let ops = diesel::update(aurach_user::table.find(id))
            .set(&user)
            .execute(conn);

        match ops {
            Ok(_) => aurach_user::table.find(id).first(conn),
            Err(e) => Err(e),
        }
    }

    pub fn delete(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(aurach_user::table.find(id))
            .execute(conn)
            .is_ok()
    }
}
