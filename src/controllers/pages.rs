
use rocket_contrib::Template;
use rocket::http::Cookies;
use controllers::context::Context;



#[get("/")]
pub fn home(cookies: Cookies) -> Template {
    let mut context = Context::new();
    context.set_current_user(cookies);
    Template::render("pages/home", &context)
}
