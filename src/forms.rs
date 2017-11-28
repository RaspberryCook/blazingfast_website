
// #[derive(Insertable)]
// #[table_name = "recipes"]
#[derive(Debug, FromForm)]
pub struct Recipe {
    pub name: String,
}


// #[derive(Insertable)]
// #[table_name = "recipes"]
#[derive(Debug, FromForm)]
pub struct User {
    pub firstname: String,
    pub lastname: String,
}
