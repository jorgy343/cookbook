use config::MongoConfig;
use mongodb::{options::ClientOptions, Client, Collection};
use rocket_okapi::{
    okapi::openapi3::Responses,
    rapidoc::{make_rapidoc, GeneralConfig, RapiDocConfig},
    response::OpenApiResponderInner,
    settings::UrlObject,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};
use rocket_sample_data::recipe_repository::RecipeRepository;

mod config;
mod recipes;
mod recipes_models;

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

    let recipe_collection: Collection<rocket_sample_data::recipe::RecipeDataModel> = mongo_client
        .database(&mongo_config.database_name)
        .collection(&mongo_config.recipe_collection_name);

    rocket
        .manage(Box::new(recipe_collection) as Box<dyn RecipeRepository>)
        .mount("/swagger", make_swagger_ui(&get_swagger_config()))
        .mount("/rapi-doc", make_rapidoc(&get_rapidoc_config()))
        .attach(recipes::stage())
}

fn get_swagger_config() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/open-api/openapi.json".to_string(),
        ..Default::default()
    }
}

fn get_rapidoc_config() -> RapiDocConfig {
    RapiDocConfig {
        general: GeneralConfig {
            spec_urls: vec![UrlObject::new("Resource", "/open-api/openapi.json")],
            ..Default::default()
        },
        ..Default::default()
    }
}

#[derive(Debug, Responder)]
pub enum ApiError {
    GeneralError(String),
    DatabaseError(String),
    AuthError(String),
}

impl OpenApiResponderInner for ApiError {
    fn responses(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
    ) -> rocket_okapi::Result<Responses> {
        Ok(Responses::default())
    }
}
