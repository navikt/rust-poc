#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod api;
mod environment;
mod structs;

use api::api;
use environment::Environment;
use structs::blueprint::Blueprint;


fn main() {
    let api = api();
    let _env = Environment::new();

    let mut app = rocket::ignite();
    for route in api {
        app = app.mount(route.basepath, route.paths);
    }
    app.launch();
}