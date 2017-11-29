
use diesel::{LimitDsl, LoadDsl, FilterDsl, ExpressionMethods};
use database;
use models::user::User;
use diesel::result::Error;
use schema;
use forms;


#[derive(FromForm)]
pub struct Session {
    pub email: String,
    pub password: String,
}


impl Session {
    pub fn user(&self) -> Result<User, Error> {

        let encrypted_password = forms::user::User::encrypt(&self.password);

        let connection = database::establish_connection();
        let result = schema::users::dsl::users
            .filter(schema::users::dsl::email.eq(&self.email))
            .filter(schema::users::dsl::password.eq(encrypted_password))
            .limit(1)
            .load::<User>(&connection);

        let users = match result {
            Ok(users) => users,
            Err(_) => return Err(Error::NotFound),
        };

        match users.first() {
            Some(ref mut user) => Ok(user.clone()),
            None => Err(Error::NotFound),
        }
    }
}
