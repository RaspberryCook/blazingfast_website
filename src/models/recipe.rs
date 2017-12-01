// use schema::recipes::dsl::*;
// use diesel::prelude::*;
use std;

use models;
use models::user::User;
use database;
use schema;
use schema::recipes;
use schema::recipes::dsl::recipes as table;
use diesel;
use diesel::{LimitDsl, LoadDsl, FilterDsl, FindDsl, ExpressionMethods, ExecuteDsl};


#[derive(Serialize, Queryable, Clone, Identifiable)]
#[table_name = "recipes"]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
}


impl Recipe {
    // Get all recipes
    pub fn all(limit: i64) -> std::vec::Vec<Self> {
        let connection = database::establish_connection();
        table.limit(limit).load::<Self>(&connection).expect(
            "Error loading recipes",
        )
    }

    /// Find recipe by it's id
    pub fn find(recipe_id: i32) -> Self {
        let connection = database::establish_connection();
        let result = table
            .filter(schema::recipes::dsl::id.eq(recipe_id))
            .limit(1)
            .load::<Self>(&connection)
            .expect("Cannot find recipe");

        match result.first() {
            Some(ref mut recipe) => recipe.clone(),
            None => panic!("Cannot find recipe"),
        }
    }

    /// remove
    pub fn delete(&self) -> bool {
        let connection = database::establish_connection();

        match diesel::delete(table.find(self.id)).execute(&connection) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn user(&self) -> models::user::User {
        User::find(self.user_id)
    }

    pub fn is_owned_by(&self, user: &User) -> bool {
        user.id != self.user_id
    }
}
