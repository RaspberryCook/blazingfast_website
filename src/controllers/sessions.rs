
use rocket_contrib::Template;
use rocket::response::Redirect;

#[get("/new")]
pub fn new() -> Template {
    Template::render("sessions/new", &())
}

#[post("/")]
pub fn create() -> Redirect {
    // todo: create connection
    Redirect::to("/")
}
