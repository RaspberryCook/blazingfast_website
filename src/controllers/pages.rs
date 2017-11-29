
use rocket_contrib::Template;
use rocket::http::{Cookie, Cookies};

#[get("/")]
pub fn home(cookies: Cookies) -> Template {
    match cookies.get("user") {
        Some(ref cookie) => Template::render("pages/home", &cookie.value()),
        None => Template::render("pages/home", &()),
    }
}
