use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
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

impl From<Recipe> for rocket_sample_data::recipe::Recipe {
    fn from(value: Recipe) -> Self {
        Self {
            name: value.name,
            description: value.description,
            steps: value
                .steps
                .iter()
                .map(|recipe_step| {
                    rocket_sample_data::recipe::RecipeStep::from(recipe_step.clone())
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
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

impl From<RecipeStep> for rocket_sample_data::recipe::RecipeStep {
    fn from(value: RecipeStep) -> Self {
        Self {
            description: value.description,
        }
    }
}
