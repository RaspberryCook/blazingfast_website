
use rocket_contrib::Template;
use rocket::http::{Cookie, Cookies};
use controllers::context::Context;



#[get("/")]
pub fn home(cookies: Cookies) -> Template {
    let mut context = Context::new();
    context.add_current_user(cookies);
    Template::render("pages/home", &context)
}
