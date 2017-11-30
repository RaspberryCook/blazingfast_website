
// #![feature(plugin, decl_macro)]
// #![plugin(rocket_codegen)]

extern crate rocket;

use std::fmt;
use rocket::request::{self, Request, FromRequest};
use rocket::outcome::Outcome::*;
// use rocket::http::Cookies;
use rocket::http::Status;

#[derive(Debug, Serialize)]
pub struct Session(String);

impl fmt::Display for Session {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Session {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        match request.cookies().get("user") {
            // Some(ref cookie) => Template::render("pages/home", &cookie.value()),
            Some(ref cookie) => Success(Session(cookie.value().to_string())),
            None => Failure((Status::Forbidden, ())),
        }
    }
}
