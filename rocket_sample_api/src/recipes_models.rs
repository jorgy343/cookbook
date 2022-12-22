use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema)]
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

#[derive(Serialize, Deserialize, JsonSchema)]
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
