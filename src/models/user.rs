#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
}
