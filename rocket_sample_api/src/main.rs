use config::MongoConfig;
use mongodb::{options::ClientOptions, Client, Collection};
use rocket_sample_data::recipe_repository::RecipeRepository;

mod config;
mod recipes;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    let rocket = rocket::build();
    let figment = rocket.figment();

    let mongo_config: MongoConfig = figment
        .extract_inner("mongo")
        .expect("Failed to parse the mongo configuration.");

    let mongo_client_options = ClientOptions::parse(mongo_config.connection_string)
        .await
        .expect("Failed to parse Mongo options.");

    let mongo_client =
        Client::with_options(mongo_client_options).expect("Failed to create the Mongo client.");

    let recipe_collection: Collection<rocket_sample_data::recipe::Recipe> = mongo_client
        .database(&mongo_config.database_name)
        .collection(&mongo_config.recipe_collection_name);

    rocket
        .manage(Box::new(recipe_collection) as Box<dyn RecipeRepository>)
        .attach(recipes::stage())
}

#[derive(Debug, Responder)]
pub enum ApiError {
    GeneralError(String),
    DatabaseError(String),
}
