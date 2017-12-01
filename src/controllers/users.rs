
use rocket_contrib::Template;
use rocket::response::Redirect;
use rocket::http::Cookies;
use rocket::request::Form;
use schema::users::dsl::*;
use diesel;
use diesel::prelude::*;
use middlewares::session::Session;
use controllers::context::Context;

use models::user::User;
use forms::user::User as user_form;
use database;


/// List all users
#[get("/")]
pub fn index(cookies: Cookies) -> Template {
    let mut context = Context::new();
    context.set_current_user(cookies);
    context.load_users();

    Template::render("users/index", &context)
}

/// Show user
#[get("/<user_id>")]
pub fn show(user_id: i32, cookies: Cookies) -> Template {
    let user = User::find(user_id);
    let user_recipes = user.recipes();
    let mut context = Context::new();
    context.set_current_user(cookies);
    context.set_recipes(user_recipes);
    context.editable = match context.get_current_user() {
        Some(current_user) => (current_user.id == user.id),
        None => false,
    };
    context.set_user(user);

    Template::render("users/show", &context)
}


#[get("/new")]
pub fn new() -> Template {
    Template::render("users/new", &())
}

#[post("/", data = "<form_data>")]
pub fn create(form_data: Form<user_form>) -> Redirect {
    let connection = database::establish_connection();
    let user = user_form::from_form(form_data);

    match diesel::insert(&user).into(users).execute(&connection) {
        Ok(_) => Redirect::to("/users"),
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    }
}

#[get("/<user_id>/edit")]
pub fn edit(user_id: i32, cookies: Cookies) -> Template {
    let user = User::find(user_id);
    let mut context = Context::new();
    context.set_current_user(cookies);

    // get user & redirect if don't exist
    let current_user = match context.get_current_user() {
        Some(current_user) => current_user,
        None => return Template::render("errors/403", &context),
    };

    // verify owner
    if current_user.id == user.id {
        context.set_user(user);
        Template::render("users/edit", &context)
    } else {
        Template::render("errors/403", &context)
    }
}

#[put("/<user_id>", data = "<form_data>")]
pub fn update(user_id: i32, form_data: Form<user_form>, session: Session) -> Redirect {
    let current_user = session.user();
    let user = User::find(user_id);

    // check if current user is this user
    if current_user.id == user.id {
        // TODO: build a return to errors#403
        panic!("Can't delete recipe")
    }

    // update user
    let connection = database::establish_connection();
    let result = diesel::update(users.find(user_id))
        .set((
            firstname.eq(form_data.get().firstname.to_string()),
            lastname.eq(form_data.get().lastname.to_string()),
        ))
        .execute(&connection);

    match result {
        Ok(_) => Redirect::to(&format!("/users/{}", user_id)),
        Err(error) => panic!("{:?}", error),
    }
}

#[delete("/<user_id>")]
pub fn delete(session: Session, user_id: i32) -> Redirect {
    let current_user = session.user();
    let user = User::find(user_id);

    // check if current user is this user
    if current_user.id == user.id {
        // TODO: build a return to errors#403
        panic!("Can't delete recipe")
    }

    if User::find(user_id).delete() {
        Redirect::to("/users")
    } else {
        panic!("Can't delete user")
    }
}
