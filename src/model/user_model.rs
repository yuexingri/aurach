use rocket::response::Responder;
use rocket::{Request, response, Response};
use rocket::http::ContentType;
use std::io::Cursor;
use rocket::logger::error;

#[derive(Debug)]
pub struct User {
    id: u8,
    name: String,
}

impl User {

    pub fn new_user(id: u8, name: String) -> User {
        User {
            id,
            name,
        }
    }

}

impl<'r> Responder<'r> for User {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .header(ContentType::Plain)
            .sized_body(Cursor::new(format!("{} {}", self.name, self.id)))
            .ok()
    }
}

pub fn get_user_by_id_from_db(id: u8) -> User {
    let users = vec![User {
        id: 1,
        name: "tom".to_owned(),
    },
    User {
        id: 2,
        name: "jerry".to_owned(),
    }];

    for user in users.iter() {
        if user.id == id {
            return User::new_user(user.id, user.name.to_owned());
        }
    }

    panic!("not found".to_owned());

}