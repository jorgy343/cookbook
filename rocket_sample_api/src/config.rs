use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MongoConfig {
    pub connection_string: String,
    pub database_name: String,
    pub recipe_collection_name: String,
}
