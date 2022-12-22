use rocket::{
    fairing,
    http::Status,
    request::{self, FromRequest},
    serde::json::Json,
    State,
};
use rocket_okapi::{
    okapi::openapi3::{Object, SecurityRequirement, SecurityScheme, SecuritySchemeData},
    openapi, openapi_get_routes,
    request::{OpenApiFromRequest, RequestHeaderInput},
};
use rocket_sample_data::recipe_repository::RecipeRepository;

use crate::{recipes_models::Recipe, ApiError};

struct ApiKey;

#[async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiError;

    async fn from_request(request: &'r rocket::Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(api_key) = request.headers().get_one("x-api-key") {
            if api_key == "TEST" {
                return request::Outcome::Success(ApiKey);
            }
        }

        request::Outcome::Failure((
            Status::Unauthorized,
            ApiError::AuthError("Bad auth!".to_string()),
        ))
    }
}

impl<'r> OpenApiFromRequest<'r> for ApiKey {
    fn from_request_input(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let mut security_requirements = SecurityRequirement::new();
        security_requirements.insert("ApiKeyAuth".to_string(), vec![]);

        Ok(RequestHeaderInput::Security(
            "ApiKeyAuth".to_string(),
            SecurityScheme {
                description: None,
                data: SecuritySchemeData::ApiKey {
                    name: "x-api-key".to_string(),
                    location: "header".to_string(),
                },
                extensions: Object::default(),
            },
            security_requirements,
        ))
    }
}

#[openapi]
#[get("/<recipe_name>")]
async fn get(
    _api_key: ApiKey,
    recipe_name: String,
    recipe_repository: &State<Box<dyn RecipeRepository>>,
) -> Result<Option<Json<Recipe>>, ApiError> {
    if let Some(recipe) = recipe_repository
        .get(&recipe_name)
        .await
        .map_err(|e| ApiError::DatabaseError(e.kind.to_string()))?
    {
        return Ok(Some(Json(Recipe::from(recipe))));
    }

    Ok(None)
}

// #[openapi]
// #[post("/", data = "<recipe>")]
// async fn create(
//     _api_key: ApiKey,
//     recipe: Json<Recipe>,
//     recipe_repository: &State<Box<dyn RecipeRepository>>,
// ) -> Result<Json<Recipe>, ApiError> {
//     let data_recipe: rocket_sample_data::recipe::Recipe = Recipe::into(recipe.0);

//     Ok(recipe)
// }

pub fn stage() -> fairing::AdHoc {
    fairing::AdHoc::on_ignite("Recipes", |rocket| async {
        rocket
            .mount("/recipes", routes![get])
            .mount("/open-api", openapi_get_routes![get])
    })
}
