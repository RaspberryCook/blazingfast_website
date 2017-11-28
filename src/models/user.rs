use std;

use database;

use schema::users::dsl::users;
use diesel::LimitDsl;
use diesel::LoadDsl;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
}

impl User {
    // Get all recipes
    pub fn all(limit: i64) -> std::vec::Vec<Self> {
        let connection = database::establish_connection();
        users.limit(limit).load::<Self>(&connection).expect(
            "Error loading users",
        )
    }
}
