use async_trait::async_trait;
use mongodb::{bson::doc, Collection};

use crate::recipe::RecipeDataModel;

#[async_trait]
pub trait RecipeRepository: Sync + Send {
    async fn get(&self, name: &str) -> Result<Option<RecipeDataModel>, mongodb::error::Error>;
    async fn create(&self, recipe: &RecipeDataModel) -> Result<(), mongodb::error::Error>;
    async fn update(&self, recipe: &RecipeDataModel) -> Result<(), mongodb::error::Error>;
    async fn delete(&self, name: &str) -> Result<(), mongodb::error::Error>;
}

#[async_trait]
impl RecipeRepository for Collection<RecipeDataModel> {
    async fn get(&self, name: &str) -> Result<Option<RecipeDataModel>, mongodb::error::Error> {
        let filter = doc! {"name": name};

        self.find_one(filter, None).await
    }

    async fn create(&self, recipe: &RecipeDataModel) -> Result<(), mongodb::error::Error> {
        self.insert_one(recipe, None).await?;
        Ok(())
    }

    async fn update(&self, recipe: &RecipeDataModel) -> Result<(), mongodb::error::Error> {
        let filter = doc! {"name": &recipe.name};

        self.replace_one(filter, recipe, None).await?;
        Ok(())
    }

    async fn delete(&self, name: &str) -> Result<(), mongodb::error::Error> {
        let filter = doc! {"name": name};

        self.delete_one(filter, None).await?;
        Ok(())
    }
}
