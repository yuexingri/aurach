use rocket::response::Responder;
use rocket::{Request, response, Response};
use rocket::http::ContentType;
use std::io::Cursor;

#[derive(Debug)]
pub struct User {
    id: u8,
    name: String,
}

impl<'r> Responder<'r> for User {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .header(ContentType::Plain)
            .sized_body(Cursor::new(format!("{} {}", self.name, self.id)))
            .ok()
    }
}

#[get("/")]
pub fn get() -> User {
    User {
        id: 2,
        name: String::from("jerry").to_owned(),
    }
}