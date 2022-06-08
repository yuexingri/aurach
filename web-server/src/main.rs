#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use rocket::Route;

// use storage::StorageObject;

mod configuration;
mod controller;
mod db;
mod model;
mod service;

fn main() {

    rocket::ignite()
        .manage(db::mysql_db_pool::connect())
        .mount("/user", build_user_controller_method_routes_array())
        .launch();
}

fn build_user_controller_method_routes_array() -> Vec<Route> {
    routes![
        controller::user_controller::get_by_id,
        controller::user_controller::update_by_id,
    ]
}
