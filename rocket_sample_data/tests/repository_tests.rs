use mongodb::{Client, Collection};
use rocket_sample_data::{
    recipe::Recipe,
    repository::{MongoRecipeRepository, RecipeRepository},
};

#[tokio::test]
async fn test_create_integration() {
    // Connect to a real MongoDB instance.
    let client = Client::with_uri_str("mongodb://localhost").await.unwrap();
    let collection: Collection<Recipe> = client.database("test_db").collection("recipes");

    // Insert a test recipe.
    let recipe = Recipe::new(1, "Cake", "One delicious cake.");
    let repository = MongoRecipeRepository::new(client, collection);

    let result = repository.create(&recipe).await;

    assert!(result.is_ok());

    // Verify that the recipe was inserted successfully.
    // let find_result = collection.find_one(None, None).await;
    // assert!(find_result.is_ok());
    // let find_result = find_result.unwrap();
    // assert!(find_result.is_some());
    // let find_result = find_result.unwrap();
    // let inserted_recipe: Recipe = bson::from_bson(bson::Bson::Document(find_result)).unwrap();

    // assert_eq!(inserted_recipe, "recipe");
}