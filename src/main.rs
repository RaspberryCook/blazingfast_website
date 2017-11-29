#![feature(plugin)]
#![feature(custom_derive)]
#![feature(custom_attribute)]
#![plugin(rocket_codegen, diesel_codegen)]
extern crate crypto;
extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;

use rocket_contrib::Template;


mod controllers;
mod models;
mod forms;
mod database;
mod schema;


fn main() {
    rocket::ignite()
        .mount("/", routes![controllers::pages::home])
        // recipes
        .mount("/recipes", routes![controllers::recipes::create])
        .mount("/recipes", routes![controllers::recipes::index])
        .mount("/recipes", routes![controllers::recipes::show])
        .mount("/recipes", routes![controllers::recipes::new])
        .mount("/recipes", routes![controllers::recipes::edit])
        .mount("/recipes", routes![controllers::recipes::update])
        .mount("/recipes", routes![controllers::recipes::delete])
        // users
        .mount("/users", routes![controllers::users::create])
        .mount("/users", routes![controllers::users::index])
        .mount("/users", routes![controllers::users::show])
        .mount("/users", routes![controllers::users::new])
        .mount("/users", routes![controllers::users::edit])
        .mount("/users", routes![controllers::users::update])
        .mount("/users", routes![controllers::users::delete])
        // session
        .mount("/sessions", routes![controllers::sessions::new])
        .mount("/sessions", routes![controllers::sessions::create])
        .attach(Template::fairing())
        .launch();
}
