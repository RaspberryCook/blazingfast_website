#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen, diesel_codegen)]
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
        .mount("/recipes", routes![controllers::recipes::create])
        .mount("/recipes", routes![controllers::recipes::index])
        .mount("/recipes", routes![controllers::recipes::show])
        .mount("/recipes", routes![controllers::recipes::new])
        .mount("/recipes", routes![controllers::recipes::edit])
        .mount("/recipes", routes![controllers::recipes::update])
        .mount("/recipes", routes![controllers::recipes::delete])
        .attach(Template::fairing())
        .launch();
}
