#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

// use storage::StorageObject;

mod configuration;
mod controller;
mod db;
mod model;
mod service;

fn main() {
    rocket::ignite()
        .manage(db::mysql_db_pool::connect())
        .mount("/user", routes![controller::user_controller::get])
        .launch();
}
