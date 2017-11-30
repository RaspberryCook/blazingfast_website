
use rocket_contrib::Template;
use middlewares::session::Session;

#[get("/")]
pub fn home(session: Session) -> Template {
    Template::render("pages/home", &session)
}
