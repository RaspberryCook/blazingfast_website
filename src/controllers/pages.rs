
use rocket_contrib::Template;

#[get("/")]
pub fn home() -> Template {
    Template::render("pages/home", &())
}
