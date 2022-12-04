#[allow(non_snake_case)]
pub mod CityController {
    use super::super::{
        builder::CityBuilder, entity::CreateCityPayload, model::CityModel, schema::CitySchema,
    };
    use crate::{
        config::app_state::AppState, traits::builder::Builder, utils::json_response::JsonResponse,
    };
    use actix_web::{delete, get, post, put, web, Responder};

    #[get("/api/v1/cities")]
    pub async fn get_cities(state: web::Data<AppState>) -> impl Responder {
        let pool = state.get_pool().clone();
        let result = CityModel::select(&pool, None, None).await;

        match result {
            Ok(cities) => web::Json(JsonResponse::<Vec<CitySchema>, String>::ok(
                "Success to get cities",
                Some(cities),
            )),
            Err(error) => web::Json(JsonResponse::<Vec<CitySchema>, String>::bad_request(
                "Failed to get cities",
                Some(format!("Error: {}", error)),
            )),
        }
    }

    #[get("/api/v1/cities/{id}")]
    pub async fn get_city(state: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
        let pool = state.get_pool().clone();
        let result = CityModel::detail(&pool, &id.to_string()).await;

        match result {
            Ok(city) => web::Json(JsonResponse::<CitySchema, String>::ok(
                "Success to get city",
                Some(city),
            )),
            Err(error) => web::Json(JsonResponse::<CitySchema, String>::bad_request(
                "Failed to get city",
                Some(format!("Error: {}", error)),
            )),
        }
    }

    #[post("/api/v1/cities")]
    pub async fn create_city(
        state: web::Data<AppState>,
        payload: web::Json<CreateCityPayload>,
    ) -> impl Responder {
        let pool = state.get_pool().clone();
        let city = CityBuilder::new(
            Some(payload.get_name().to_string()),
            Some(payload.get_province_id().to_string()),
        );
        let result = CityModel::insert(&pool, &city.build()).await;

        println!("{:?}", result);

        match result {
            Ok(_) => web::Json(JsonResponse::<(), String>::ok(
                "Success to create city",
                None,
            )),
            Err(_) => web::Json(JsonResponse::<(), String>::bad_request(
                "Failed to create city",
                None,
            )),
        }
    }

    #[put("/api/v1/cities/{id}")]
    pub async fn update_city(
        state: web::Data<AppState>,
        id: web::Path<String>,
        payload: web::Json<CreateCityPayload>,
    ) -> impl Responder {
        let pool = state.get_pool().clone();
        let result = CityModel::update(&pool, &id.to_string(), &payload.clone()).await;

        match result {
            Ok(_) => web::Json(JsonResponse::<(), String>::ok(
                "Success to update city",
                None,
            )),
            Err(_) => web::Json(JsonResponse::<(), String>::bad_request(
                "Failed to update city",
                None,
            )),
        }
    }

    #[delete("/api/v1/cities/{id}")]
    pub async fn delete_city(state: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
        let pool = state.get_pool().clone();
        let result = CityModel::delete(&pool, &id.to_string()).await;

        match result {
            Ok(_) => web::Json(JsonResponse::<(), String>::ok(
                "Success to delete city",
                None,
            )),
            Err(_) => web::Json(JsonResponse::<(), String>::bad_request(
                "Failed to delete city",
                None,
            )),
        }
    }
}
