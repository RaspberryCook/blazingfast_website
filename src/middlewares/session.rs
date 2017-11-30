
use std::fmt;
use rocket::request::{self, Request, FromRequest};
use rocket::outcome::Outcome::*;
use rocket::http::Status;
use models::user::User;

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
        match request.cookies().get_private("user_id") {
            Some(ref cookie) => Success(Session(cookie.value().to_string())),
            None => Failure((Status::Forbidden, ())),
        }
    }
}


impl Session {
    pub fn user(&self) -> User {
        let id = self.0.parse::<i32>().unwrap();
        User::find(id)
    }
}
