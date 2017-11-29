
use rocket_contrib::Template;
use rocket::response::Redirect;
use rocket::request::Form;
use schema::users::dsl::*;
use diesel;
use diesel::prelude::*;

use models;
use forms;
use database;

#[get("/")]
pub fn index() -> Template {
    Template::render("users/index", models::user::User::all(20))
}

#[get("/<user_id>")]
pub fn show(user_id: i32) -> Template {
    Template::render("users/show", models::user::User::find(user_id))
}


#[get("/new")]
pub fn new() -> Template {
    Template::render("users/new", &())
}

#[post("/", data = "<form_data>")]
pub fn create(form_data: Form<forms::user::User>) -> Redirect {
    let connection = database::establish_connection();
    let user = forms::user::User {
        id: None,
        firstname: form_data.get().firstname.to_string(),
        lastname: form_data.get().lastname.to_string(),
    };

    match diesel::insert(&user).into(users).execute(&connection) {
        Ok(_) => Redirect::to("/users"),
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    }
}

#[get("/<user_id>/edit")]
pub fn edit(user_id: i32) -> Template {
    Template::render("users/edit", models::user::User::find(user_id))
}

#[put("/<user_id>", data = "<form_data>")]
pub fn update(user_id: i32, form_data: Form<forms::user::User>) -> Redirect {
    let connection = database::establish_connection();

    let result = diesel::update(users.find(user_id))
        .set((
            firstname.eq(form_data.get().firstname.to_string()),
            lastname.eq(form_data.get().lastname.to_string()),
        ))
        .execute(&connection);

    match result {
        Ok(_) => Redirect::to(&format!("/users/{}", user_id)),
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    }
}

#[delete("/<user_id>")]
pub fn delete(user_id: i32) -> Redirect {
    if models::user::User::find(user_id).delete() {
        Redirect::to("/users")
    } else {
        panic!("Can't delete user")
    }
}