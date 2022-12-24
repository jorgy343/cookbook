use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct RecipeApiModel {
    pub name: String,
    pub description: String,
    pub steps: Vec<RecipeStepApiModel>,
}

impl From<rocket_sample_data::recipe::RecipeDataModel> for RecipeApiModel {
    fn from(value: rocket_sample_data::recipe::RecipeDataModel) -> Self {
        Self {
            name: value.name,
            description: value.description,
            steps: value
                .steps
                .iter()
                .map(|recipe_step| RecipeStepApiModel::from(recipe_step.clone()))
                .collect(),
        }
    }
}

impl From<RecipeApiModel> for rocket_sample_data::recipe::RecipeDataModel {
    fn from(value: RecipeApiModel) -> Self {
        Self {
            name: value.name,
            description: value.description,
            steps: value
                .steps
                .iter()
                .map(|recipe_step| {
                    rocket_sample_data::recipe::RecipeStepDataModel::from(recipe_step.clone())
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct RecipeStepApiModel {
    pub description: String,
}

impl From<rocket_sample_data::recipe::RecipeStepDataModel> for RecipeStepApiModel {
    fn from(value: rocket_sample_data::recipe::RecipeStepDataModel) -> Self {
        Self {
            description: value.description,
        }
    }
}

impl From<RecipeStepApiModel> for rocket_sample_data::recipe::RecipeStepDataModel {
    fn from(value: RecipeStepApiModel) -> Self {
        Self {
            description: value.description,
        }
    }
}
