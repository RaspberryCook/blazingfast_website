use models::user::User;
use rocket::http::Cookies;


#[derive(Serialize)]
pub struct Context {
    current_user: Option<User>,
}

impl Context {
    pub fn new() -> Context {
        Context { current_user: None }
    }

    /// Try to add current user from cookie
    pub fn add_current_user(&mut self, mut cookies: Cookies) {
        match cookies.get_private("user_id") {
            Some(ref cookie) => {
                let user_id = cookie.value().parse::<i32>().unwrap();
                let user = User::find(user_id);
                self.current_user = Some(user);
            }
            None => (),
        }
    }
}
