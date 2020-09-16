#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod api;
mod structs;

use api::Api;
use structs::blueprint::Blueprint;


fn main() {
    let api = Api::new();

    rocket::ignite()
        .mount(api.root.basepath, api.root.paths)
        .mount(api.internal.basepath, api.internal.paths)
        .launch();
}