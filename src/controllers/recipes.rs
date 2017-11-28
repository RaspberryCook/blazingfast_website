
use rocket_contrib::Template;
use rocket::response::Redirect;
use rocket::request::Form;
use schema::recipes::dsl::*;
use diesel;
use diesel::prelude::*;

use models;
use forms;
use database;


#[get("/")]
pub fn index() -> Template {
    Template::render("recipes/index", models::recipe::Recipe::all(20))
}

#[get("/<recipe_id>")]
pub fn show(recipe_id: i32) -> Template {
    Template::render("recipes/show", models::recipe::Recipe::find(recipe_id))
}


#[get("/new")]
pub fn new() -> Template {
    Template::render("recipes/new", models::user::User::all(20))
}

#[post("/", data = "<form_data>")]
pub fn create(form_data: Form<forms::recipe::Recipe>) -> Redirect {
    let connection = database::establish_connection();

    let recipe = forms::recipe::Recipe {
        id: None,
        name: form_data.get().name.to_string(),
        user_id: form_data.get().user_id,
    };

    match diesel::insert(&recipe).into(recipes).execute(&connection) {
        Ok(_) => Redirect::to("/recipes"),
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    }
}

#[get("/<recipe_id>/edit")]
pub fn edit(recipe_id: i32) -> Template {
    #[derive(Serialize)]
    struct Context {
        recipe: models::recipe::Recipe,
        users: Vec<models::user::User>,
    }


    Template::render(
        "recipes/edit",
        Context {
            recipe: models::recipe::Recipe::find(recipe_id),
            users: models::user::User::all(20),
        },
    )
}

#[put("/<recipe_id>", data = "<form_data>")]
pub fn update(recipe_id: i32, form_data: Form<forms::recipe::Recipe>) -> Redirect {
    let connection = database::establish_connection();

    let result = diesel::update(recipes.find(recipe_id))
        .set((
            name.eq(form_data.get().name.to_string()),
            // user_id.eq(form_data.get().user_id),
        ))
        .execute(&connection);


    match result {
        Ok(_) => Redirect::to(&format!("/recipes/{}", recipe_id)),
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    }
}

#[delete("/<recipe_id>")]
pub fn delete(recipe_id: i32) -> Redirect {
    if models::recipe::Recipe::find(recipe_id).delete() {
        Redirect::to("/recipes")
    } else {
        panic!("Can't delete recipe")
    }
}
