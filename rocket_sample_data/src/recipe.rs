use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeDataModel {
    pub name: String,
    pub description: String,
    pub steps: Vec<RecipeStepDataModel>,
}

impl RecipeDataModel {
    pub fn new(name: &'static str, description: &'static str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            steps: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeStepDataModel {
    pub description: String,
}

impl RecipeStepDataModel {
    pub fn new(description: String) -> Self {
        Self { description }
    }
}
