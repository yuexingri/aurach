use std::io::Cursor;

use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};
use diesel::prelude::*;
use diesel::result::Error;
use rocket::{Request, response, Response};
use rocket::http::ContentType;
use rocket::logger::error;
use rocket::response::Responder;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::model::schema::aurach_user;

#[derive(Debug)]
#[table_name = "aurach_user"]
#[derive(Queryable, Serialize, Deserialize, Insertable, AsChangeset)]
pub struct User {
    pub id: Option<i32>,
    pub name: Option<String>,
}

impl User {

    pub fn create(user: User, conn: &MysqlConnection) -> Result<User, Error> {
        let user = User {
            ..user
        };

        let ops = diesel::insert_into(aurach_user::table).values(&user).execute(conn);

        match ops {
            Ok(_) => aurach_user::table.order(aurach_user::id.desc()).first(conn),
            Err(e) => Err(e)
        }
    }

    pub fn read(conn: &MysqlConnection) -> Result<Vec<User>, Error> {
        aurach_user::table.order(aurach_user::id.asc()).load::<User>(conn)
    }

    pub fn read_by_id(id: i32, conn: &MysqlConnection) -> Result<User, Error> {
        aurach_user::table.find(id).first(conn)
    }

    pub fn update(id: i32, user: User, conn: &MysqlConnection) -> Result<User, Error> {
        let user = User {
            ..user
        };

        let ops = diesel::update(aurach_user::table.find(id)).set(&user).execute(conn);

        match ops {
            Ok(_) => aurach_user::table.find(id).first(conn),
            Err(e) => Err(e),
        }
    }

    pub fn delete(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(aurach_user::table.find(id)).execute(conn).is_ok()
    }

}
//
// impl<'r> Responder<'r> for User {
//     fn respond_to(self, _: &Request) -> response::Result<'r> {
//         Response::build()
//             .header(ContentType::Plain)
//             .sized_body(Cursor::new(format!("{} {}", self.name, self.id)))
//             .ok()
//     }
// }
//
// pub fn get_user_by_id_from_db(id: u8) -> User {
//     let users = vec![User {
//         id: 1,
//         name: "tom".to_owned(),
//     },
//     User {
//         id: 2,
//         name: "jerry".to_owned(),
//     }];
//
//     for user in users.iter() {
//         if user.id == id {
//             return User::new_user(user.id, user.name.to_owned());
//         }
//     }
//
//     panic!("not found".to_owned());
//
// }