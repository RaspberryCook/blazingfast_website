
use rocket_contrib::Template;

#[get("/")]
pub fn home() -> Template {
    let map = ();
    Template::render("pages/home", &map)
}
