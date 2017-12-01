
use rocket_contrib::Template;
use rocket::http::Cookies;
use rocket::response::Redirect;
use rocket::request::Form;
use schema::recipes::dsl::*;
use diesel;
use diesel::prelude::*;
use middlewares::session::Session;

use controllers::context::Context;


use models;
use forms;
use database;


#[get("/")]
pub fn index(cookies: Cookies) -> Template {
    let mut context = Context::new();
    context.set_current_user(cookies);
    context.load_recipes_and_user();

    Template::render("recipes/index", &context)
}

#[get("/<recipe_id>")]
pub fn show(recipe_id: i32, cookies: Cookies) -> Template {
    let recipe = models::recipe::Recipe::find(recipe_id);
    let user = recipe.user();
    let mut context = Context::new();
    context.set_current_user(cookies);
    context.editable = match context.get_current_user() {
        Some(current_user) => (current_user.id == recipe.user_id),
        None => false,
    };
    context.set_recipe(recipe);
    context.set_user(user);

    Template::render("recipes/show", &context)
}


#[get("/new")]
pub fn new(_session: Session, cookies: Cookies) -> Template {
    let mut context = Context::new();
    context.set_current_user(cookies);

    Template::render("recipes/new", &context)
}

#[post("/", data = "<form>")]
pub fn create(form: Form<forms::recipe::Recipe>, session: Session) -> Redirect {
    let recipe = form.get().new_recipe(session.user_id());
    let connection = database::establish_connection();

    match diesel::insert(&recipe).into(recipes).execute(&connection) {
        Ok(_) => Redirect::to("/recipes"),
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    }
}

#[get("/<recipe_id>/edit")]
pub fn edit(_session: Session, recipe_id: i32, cookies: Cookies) -> Template {
    let mut context = Context::new();
    context.set_current_user(cookies);
    let current_user = context.get_current_user().unwrap();
    let recipe = models::recipe::Recipe::find(recipe_id);

    // check if current user own this recipe
    if recipe.is_owned_by(&current_user) {
        return Template::render("errors/403", &());
    }
    context.set_recipe(recipe);

    Template::render("recipes/edit", &context)
}

#[put("/<recipe_id>", data = "<form_data>")]
pub fn update(
    session: Session,
    recipe_id: i32,
    form_data: Form<forms::recipe::Recipe>,
) -> Redirect {
    let current_user = session.user();
    let recipe = models::recipe::Recipe::find(recipe_id);

    // check if current user own this recipe
    if recipe.is_owned_by(&current_user) {
        // TODO: build a return to errors#403
        panic!("Can't delete recipe")
    }

    let form = form_data.get();
    let connection = database::establish_connection();

    let result = diesel::update(recipes.find(recipe_id))
        .set(name.eq(form.name.to_string()))
        .execute(&connection);

    match result {
        Ok(_) => Redirect::to(&format!("/recipes/{}", recipe_id)),
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    }
}

#[delete("/<recipe_id>")]
pub fn delete(session: Session, recipe_id: i32) -> Redirect {
    let current_user = session.user();
    let recipe = models::recipe::Recipe::find(recipe_id);

    // check if current user own this recipe
    if recipe.is_owned_by(&current_user) {
        // TODO: build a return to errors#403
        panic!("Can't delete recipe")
    }

    if recipe.delete() {
        Redirect::to("/recipes")
    } else {
        // TODO: build a return to errors#403
        panic!("Can't delete recipe")
    }
}
