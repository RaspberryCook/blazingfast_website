
use rocket_contrib::Template;
use rocket::response::Redirect;
use forms::session::Session;
use rocket::request::Form;


#[get("/new")]
pub fn new() -> Template {
    Template::render("sessions/new", &())
}

#[post("/", data = "<form_data>")]
pub fn create(form_data: Form<Session>) -> Redirect {
    let form = form_data.get();

    match form.user() {
        Ok(user) => Redirect::to("/"),
        Err(_) => Redirect::to("/sessions/new"),
    }
}
