// use schema::recipes::dsl::*;
// use diesel::prelude::*;
use std;

use database;
use diesel;
use diesel::{LimitDsl, LoadDsl, FilterDsl, FindDsl, ExpressionMethods, ExecuteDsl};
use schema::recipes::dsl::recipes;
use schema;


#[derive(Serialize, Queryable, Clone)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
}


impl Recipe {
    // Get all recipes
    pub fn all(limit: i64) -> std::vec::Vec<Self> {
        let connection = database::establish_connection();
        recipes.limit(limit).load::<Self>(&connection).expect(
            "Error loading recipes",
        )
    }

    /// Find recipe by it's id
    pub fn find(recipe_id: i32) -> Self {
        let connection = database::establish_connection();
        let result = recipes
            .filter(schema::recipes::dsl::id.eq(recipe_id))
            .limit(1)
            .load::<Recipe>(&connection)
            .expect("Cannot find recipe");

        match result.first() {
            Some(ref mut recipe) => recipe.clone(),
            None => panic!("Cannot find recipe"),
        }
    }

    /// remove
    pub fn delete(&self) -> bool {
        let connection = database::establish_connection();

        match diesel::delete(recipes.find(self.id)).execute(&connection) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
