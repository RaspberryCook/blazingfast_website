
use rocket_contrib::Template;
use rocket::response::Redirect;
use rocket::request::Form;
use schema;
use schema::recipes::dsl::*;
use diesel;
use diesel::prelude::*;
use middlewares::session::Session;

use models;
use forms;
use database;


#[get("/")]
pub fn index() -> Template {
    let connection = database::establish_connection();
    let data: Vec<(models::recipe::Recipe, models::user::User)> = recipes
        .inner_join(schema::users::table)
        .load(&connection)
        .expect("Error loading data");
    Template::render("recipes/index", data)
}

#[get("/<recipe_id>")]
pub fn show(recipe_id: i32) -> Template {
    #[derive(Serialize)]
    struct Context {
        recipe: models::recipe::Recipe,
        user: models::user::User,
    }

    let recipe = models::recipe::Recipe::find(recipe_id);
    let user = recipe.user();

    Template::render(
        "recipes/show",
        Context {
            recipe: recipe,
            user: user,
        },
    )
}


#[get("/new")]
pub fn new(session: Session) -> Template {
    let user = session.user();
    Template::render("recipes/new", user)
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
pub fn edit(session: Session, recipe_id: i32) -> Template {
    let current_user = session.user();
    let recipe = models::recipe::Recipe::find(recipe_id);

    // check if current user own this recipe
    if recipe.is_owned_by(&current_user) {
        return Template::render("errors/403", &());
    }

    #[derive(Serialize)]
    struct Context {
        recipe: models::recipe::Recipe,
        users: Vec<models::user::User>,
    }

    Template::render(
        "recipes/edit",
        Context {
            recipe: recipe,
            users: models::user::User::all(20),
        },
    )
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
