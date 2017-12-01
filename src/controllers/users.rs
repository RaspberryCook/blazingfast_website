
use rocket_contrib::Template;
use rocket::response::Redirect;
use rocket::request::Form;
use schema::users::dsl::*;
use diesel;
use diesel::prelude::*;
use middlewares::session::Session;

use models;
use models::user::User;
use forms::user::User as user_form;
use database;


/// List all users
#[get("/")]
pub fn index() -> Template {
    Template::render("users/index", User::all(20))
}

/// Show user
#[get("/<user_id>")]
pub fn show(user_id: i32) -> Template {
    // context for template
    #[derive(Serialize)]
    struct Context {
        user: User,
        // recipe of user
        recipes: Vec<models::recipe::Recipe>,
    }
    // get values for context
    let user = User::find(user_id);
    let recipes = user.recipes();

    Template::render(
        "users/show",
        Context {
            user: user,
            recipes: recipes,
        },
    )
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
pub fn edit(session: Session, user_id: i32) -> Template {
    let current_user = session.user();
    let user = User::find(user_id);

    // check if current user is this user
    if current_user.id == user.id {
        Template::render("users/edit", User::find(user_id))
    } else {
        Template::render("errors/403", &())
    }
}

#[put("/<user_id>", data = "<form_data>")]
pub fn update(session: Session, user_id: i32, form_data: Form<user_form>) -> Redirect {
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
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
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
