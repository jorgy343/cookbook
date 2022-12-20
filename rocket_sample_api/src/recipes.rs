use rocket::{
    fairing,
    serde::{json::Json, Serialize},
};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Recipe {
    id: u64,
    pub name: String,
    pub description: String,
    pub steps: Vec<RecipeStep>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RecipeStep {
    pub description: String,
}

impl Recipe {
    pub fn new(id: u64, name: &'static str, description: &'static str) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            steps: Vec::new(),
        }
    }
}

#[get("/<recipe_id>")]
fn get(recipe_id: u64) -> Json<Recipe> {
    let recipe = Recipe::new(recipe_id, "Apple", "Pick an apple from an apple tree.");

    Json(recipe)
}

pub fn stage() -> fairing::AdHoc {
    fairing::AdHoc::on_ignite("Recipes", |rocket| async {
        rocket.mount("/recipes", routes![get])
    })
}
