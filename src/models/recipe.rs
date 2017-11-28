#[derive(Serialize, Queryable)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
}
