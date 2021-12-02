#![feature(proc_macro_hygiene, decl_macro)]
#![feature(in_band_lifetimes)]

#[macro_use] extern crate rocket;
pub mod controller;
pub mod service;
pub mod model;
use storage::StorageObject;


fn main() {
    let storage_obj = StorageObject::new_storage_obj(String::from("tom").to_owned());
    storage_obj.say_hello();
    rocket::ignite().mount("/user", routes![controller::user_controller::get]).launch();
}
