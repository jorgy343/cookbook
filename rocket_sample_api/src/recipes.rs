use rocket::{
    fairing,
    serde::{json::Json, Serialize},
    State,
};
use rocket_sample_data::recipe_repository::RecipeRepository;

use crate::ApiError;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Recipe {
    pub name: String,
    pub description: String,
    pub steps: Vec<RecipeStep>,
}

impl From<rocket_sample_data::recipe::Recipe> for Recipe {
    fn from(value: rocket_sample_data::recipe::Recipe) -> Self {
        Self {
            name: value.name,
            description: value.description,
            steps: value
                .steps
                .iter()
                .map(|recipe_step| RecipeStep::from(recipe_step.clone()))
                .collect(),
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RecipeStep {
    pub description: String,
}

impl From<rocket_sample_data::recipe::RecipeStep> for RecipeStep {
    fn from(value: rocket_sample_data::recipe::RecipeStep) -> Self {
        Self {
            description: value.description,
        }
    }
}

#[get("/<recipe_name>")]
async fn get(
    recipe_name: String,
    recipe_repository: &State<Box<dyn RecipeRepository>>,
) -> Result<Option<Json<Recipe>>, ApiError> {
    if let Some(recipe) = recipe_repository
        .get(&recipe_name)
        .await
        .map_err(|e| ApiError::DatabaseError(e.kind.to_string()))?
    {
        return Ok(Some(Json(Recipe::from(recipe))));
    }

    Ok(None)
}

pub fn stage() -> fairing::AdHoc {
    fairing::AdHoc::on_ignite("Recipes", |rocket| async {
        rocket.mount("/recipes", routes![get])
    })
}
