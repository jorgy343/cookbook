use async_trait::async_trait;
use mongodb::{Client, Collection};

use crate::recipe::Recipe;

#[async_trait]
pub trait RecipeRepository {
    async fn create(&self, recipe: &Recipe) -> Result<(), mongodb::error::Error>;
    fn update(&self, id: u64, recipe: Recipe) -> Result<(), mongodb::error::Error>;
    fn delete(&self, id: u64) -> Result<(), mongodb::error::Error>;
    fn get(&self, id: u64) -> Result<Option<Recipe>, mongodb::error::Error>;
}

pub struct MongoRecipeRepository {
    client: Client,
    collection: Collection<Recipe>,
}

impl MongoRecipeRepository {
    pub fn new(client: Client, collection: Collection<Recipe>) -> Self {
        Self { client, collection }
    }
}

#[async_trait]
impl RecipeRepository for MongoRecipeRepository {
    async fn create(&self, recipe: &Recipe) -> Result<(), mongodb::error::Error> {
        self.collection.insert_one(recipe, None).await?;
        Ok(())
    }

    fn update(&self, id: u64, recipe: Recipe) -> Result<(), mongodb::error::Error> {
        todo!()
    }

    fn delete(&self, id: u64) -> Result<(), mongodb::error::Error> {
        todo!()
    }

    fn get(&self, id: u64) -> Result<Option<Recipe>, mongodb::error::Error> {
        todo!()
    }
}
