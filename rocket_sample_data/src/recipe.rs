use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    id: u64,
    pub name: String,
    pub description: String,
    pub steps: Vec<RecipeStep>,
}

#[derive(Debug, Serialize, Deserialize)]
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
