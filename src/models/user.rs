use std;
use database;
use schema;
use schema::users::dsl::users;
use diesel;
use diesel::{LimitDsl, LoadDsl, ExpressionMethods, FilterDsl, FindDsl, ExecuteDsl};

#[derive(Serialize, Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
}

impl User {
    // Get all users
    pub fn all(limit: i64) -> std::vec::Vec<Self> {
        let connection = database::establish_connection();
        users.limit(limit).load::<Self>(&connection).expect(
            "Error loading users",
        )
    }

    /// Find user by it's id
    pub fn find(user_id: i32) -> Self {
        let connection = database::establish_connection();
        let result = users
            .filter(schema::users::dsl::id.eq(user_id))
            .limit(1)
            .load::<Self>(&connection)
            .expect("Cannot find user");

        match result.first() {
            Some(ref mut user) => user.clone(),
            None => panic!("Cannot find user"),
        }
    }

    /// remove entry from database
    pub fn delete(&self) -> bool {
        let connection = database::establish_connection();

        match diesel::delete(users.find(self.id)).execute(&connection) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
