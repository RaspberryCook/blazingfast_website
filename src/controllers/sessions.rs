
use forms::session::Session;
use rocket::response::Redirect;
use rocket::request::Form;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::Template;


#[get("/new")]
pub fn new() -> Template {
    Template::render("sessions/new", &())
}

#[post("/", data = "<form_data>")]
pub fn create(mut cookies: Cookies, form_data: Form<Session>) -> Redirect {
    let form = form_data.get();

    match form.user() {
        Ok(user) => {
            cookies.add_private(Cookie::new("user_id", user.id.to_string()));
            Redirect::to("/")
        }
        Err(_) => Redirect::to("/sessions/new"),
    }
}
