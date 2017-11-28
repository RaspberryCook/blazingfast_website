
// #[derive(Insertable)]
// #[table_name = "recipes"]
#[derive(Debug, FromForm)]
pub struct Recipe {
    pub name: String,
    pub user_id: i32,
}


// #[derive(Insertable)]
// #[table_name = "recipes"]
#[derive(Debug, FromForm)]
pub struct User {
    pub firstname: String,
    pub lastname: String,
}
