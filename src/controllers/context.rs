use models::user::User;
use rocket::http::Cookies;
use models::recipe::Recipe;
use database;
use schema;
use schema::recipes::dsl::recipes;
use diesel::prelude::*;


#[derive(Serialize)]
pub struct Context {
    pub editable: bool,
    current_user: Option<User>,
    user: Option<User>,
    users: Option<Vec<User>>,
    recipe: Option<Recipe>,
    recipes: Option<Vec<Recipe>>,
    recipes_and_user: Option<Vec<(Recipe, User)>>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            editable: false,
            current_user: None,
            recipe: None,
            recipes: None,
            recipes_and_user: None,
            user: None,
            users: None,
        }
    }

    /// Try to add current user from cookie
    pub fn add_current_user(&mut self, mut cookies: Cookies) {
        match cookies.get_private("user_id") {
            Some(ref cookie) => {
                let user_id = cookie.value().parse::<i32>().unwrap();
                let user = User::find(user_id);
                self.current_user = Some(user);
            }
            None => (),
        }
    }

    pub fn get_current_user(&self) -> Option<User> {
        self.current_user.clone()
    }

    pub fn add_recipe(&mut self, recipe: Recipe) {
        self.recipe = Some(recipe);
    }

    pub fn add_recipes(&mut self, recipes_param: Vec<Recipe>) {
        self.recipes = Some(recipes_param);
    }

    pub fn add_user(&mut self, user: User) {
        self.user = Some(user);
    }

    pub fn load_recipes_and_user(&mut self) {
        let connection = database::establish_connection();
        self.recipes_and_user = Some(
            recipes
                .inner_join(schema::users::table)
                .load(&connection)
                .expect("Error loading data"),
        );
    }

    pub fn load_users(&mut self) {
        self.users = Some(User::all(20));
    }
}
