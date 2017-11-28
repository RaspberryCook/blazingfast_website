use super::super::schema::users;

#[table_name = "users"]
#[derive(FromForm, Insertable)]
pub struct User {
    pub id: Option<i32>,
    pub firstname: String,
    pub lastname: String,
}
