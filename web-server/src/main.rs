#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate chrono;
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
        .mount("/task_info", build_task_info_controller_method_routes_array())
        .launch();
}

fn build_user_controller_method_routes_array() -> Vec<Route> {
    routes![
        controller::user_controller::get_by_id,
        controller::user_controller::update_by_id,
        controller::user_controller::create_user,
    ]
}

fn build_task_info_controller_method_routes_array() -> Vec<Route> {
    routes![
        controller::task_info_controller::get_by_id,
    ]
}
