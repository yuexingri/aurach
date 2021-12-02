#![feature(proc_macro_hygiene, decl_macro)]
#![feature(in_band_lifetimes)]

#[macro_use] extern crate rocket;
pub mod controller;
// use controller::UserManagementController;


fn main() {
    rocket::ignite().mount("/user", routes![controller::user_controller::get]).launch();
}
