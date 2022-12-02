#[allow(non_snake_case)]
pub mod CountryController {
    use super::super::model::CountryModel;
    use crate::{
        config::app_state::AppState,
        modules::country::{
            builder::CountryBuilder, entity::CreateCountryPayload, schema::CountrySchema,
        },
        traits::builder::Builder,
        utils::json_response::JsonResponse,
    };
    use actix_web::{delete, get, post, put, web, Responder};

    #[get("/api/v1/countries")]
    pub async fn get_countries(state: web::Data<AppState>) -> impl Responder {
        let pool = state.get_pool().clone();
        let result = CountryModel::select(&pool, None, None).await;

        match result {
            Ok(countries) => web::Json(JsonResponse::<Vec<CountrySchema>, String>::ok(
                "Success to get countries",
                Some(countries),
            )),
            Err(error) => web::Json(JsonResponse::<Vec<CountrySchema>, String>::bad_request(
                "Failed to get countries",
                Some(format!("Error: {}", error)),
            )),
        }
    }

    #[get("/api/v1/countries/{id}")]
    pub async fn get_country(state: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
        let pool = state.get_pool().clone();
        let result = CountryModel::detail(&pool, &id.to_string()).await;

        match result {
            Ok(country) => web::Json(JsonResponse::<CountrySchema, String>::ok(
                "Success to get country",
                Some(country),
            )),
            Err(error) => web::Json(JsonResponse::<CountrySchema, String>::bad_request(
                "Failed to get country",
                Some(format!("Error: {}", error)),
            )),
        }
    }

    #[post("/api/v1/countries")]
    pub async fn create_country(
        state: web::Data<AppState>,
        payload: web::Json<CreateCountryPayload>,
    ) -> impl Responder {
        let pool = state.get_pool().clone();
        let country = CountryBuilder::new(
            Some(payload.get_name().to_string()),
            Some(payload.get_code().to_string()),
        );
        let result = CountryModel::insert(&pool, &country.build()).await;

        match result {
            Ok(_) => web::Json(JsonResponse::<(), String>::ok(
                "Success to create country",
                None,
            )),
            Err(_) => web::Json(JsonResponse::<(), String>::bad_request(
                "Failed to create country",
                None,
            )),
        }
    }

    #[put("/api/v1/countries/{id}")]
    pub async fn update_country(
        state: web::Data<AppState>,
        id: web::Path<String>,
        payload: web::Json<CreateCountryPayload>,
    ) -> impl Responder {
        let pool = state.get_pool().clone();
        let result = CountryModel::update(&pool, &id.to_string(), &payload.clone()).await;

        match result {
            Ok(_) => web::Json(JsonResponse::<(), String>::ok(
                "Success to update country",
                None,
            )),
            Err(_) => web::Json(JsonResponse::<(), String>::bad_request(
                "Failed to update country",
                None,
            )),
        }
    }

    #[delete("/api/v1/countries/{id}")]
    pub async fn delete_country(
        state: web::Data<AppState>,
        id: web::Path<String>,
    ) -> impl Responder {
        let pool = state.get_pool().clone();
        let result = CountryModel::delete(&pool, &id.to_string()).await;

        match result {
            Ok(_) => web::Json(JsonResponse::<(), String>::ok(
                "Success to delete country",
                None,
            )),
            Err(_) => web::Json(JsonResponse::<(), String>::bad_request(
                "Failed to delete country",
                None,
            )),
        }
    }
}
