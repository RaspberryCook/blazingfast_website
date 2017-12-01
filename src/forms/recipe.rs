use super::super::schema::recipes;

#[derive(FromForm)]
pub struct Recipe {
    pub name: String,
}


impl Recipe {
    pub fn new_recipe(&self, user_id: i32) -> NewRecipe {
        NewRecipe {
            id: None,
            name: self.name.to_string(),
            user_id: user_id,
        }
    }
}


#[table_name = "recipes"]
#[derive(Insertable)]
pub struct NewRecipe {
    pub id: Option<i32>,
    pub name: String,
    pub user_id: i32,
}
