
use rocket::request::{self, Request, FromRequest};
use rocket::outcome::Outcome::*;
use rocket::http::Status;
use models::user::User;

#[derive(Debug, Serialize)]
pub struct Session(String);

impl<'a, 'r> FromRequest<'a, 'r> for Session {
    type Error = ();
    /// check if cookie contains an user
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        match request.cookies().get_private("user_id") {
            Some(ref cookie) => Success(Session(cookie.value().to_string())),
            None => Failure((Status::Forbidden, ())),
        }
    }
}


impl Session {
    /// get user_id from cookie value
    pub fn user_id(&self) -> i32 {
        self.0.parse::<i32>().unwrap()
    }

    /// get user from cookie value
    pub fn user(&self) -> User {
        User::find(self.user_id())
    }
}
