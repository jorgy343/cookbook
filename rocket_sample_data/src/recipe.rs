use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub description: String,
    pub steps: Vec<RecipeStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeStep {
    pub description: String,
}

impl Recipe {
    pub fn new(name: &'static str, description: &'static str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            steps: Vec::new(),
        }
    }
}
