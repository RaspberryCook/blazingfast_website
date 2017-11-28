use super::super::schema::recipes;

#[derive(Serialize, Queryable)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
}

#[table_name = "recipes"]
#[derive(Insertable)]
pub struct NewRecipe {
    pub id: Option<i32>,
    pub name: String,
    pub user_id: i32,
}
