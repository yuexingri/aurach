use std::ops::Deref;

use diesel::MysqlConnection;
use dotenv::dotenv;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{Request, State};

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn connect() -> Pool {
    dotenv().ok();

    let database_url = "mysql://root:root123456@localhost/aurach";
    // let database_url = "mysql://root:root@123@localhost/aurach";

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

pub struct Connection(pub r2d2::PooledConnection<ConnectionManager<MysqlConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;

        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Connection {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
