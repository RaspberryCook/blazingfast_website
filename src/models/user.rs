use super::super::schema::users;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
}

#[table_name = "users"]
#[derive(Insertable, AsChangeset)]
pub struct NewUser {
    pub id: Option<i32>,
    pub firstname: String,
    pub lastname: String,
}
