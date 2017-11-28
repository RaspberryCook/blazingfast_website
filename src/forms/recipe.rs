use super::super::schema::recipes;

#[table_name = "recipes"]
#[derive(FromForm, Insertable)]
pub struct Recipe {
    pub id: Option<i32>,
    pub name: String,
    pub user_id: i32,
}
