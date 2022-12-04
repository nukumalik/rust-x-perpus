#[allow(non_snake_case)]
pub mod DistrictController {
    use super::super::{
        builder::DistrictBuilder, entity::CreateDistrictPayload, model::DistrictModel,
        schema::DistrictSchema,
    };
    use crate::{
        config::app_state::AppState, traits::builder::Builder, utils::json_response::JsonResponse,
    };
    use actix_web::{delete, get, post, put, web, Responder};

    #[get("/api/v1/districts")]
    pub async fn get_districts(state: web::Data<AppState>) -> impl Responder {
        let pool = state.get_pool().clone();
        let result = DistrictModel::select(&pool, None, None).await;

        match result {
            Ok(districts) => web::Json(JsonResponse::<Vec<DistrictSchema>, String>::ok(
                "Success to get districts",
                Some(districts),
            )),
            Err(error) => web::Json(JsonResponse::<Vec<DistrictSchema>, String>::bad_request(
                "Failed to get districts",
                Some(format!("Error: {}", error)),
            )),
        }
    }

    #[get("/api/v1/districts/{id}")]
    pub async fn get_district(state: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
        let pool = state.get_pool().clone();
        let result = DistrictModel::detail(&pool, &id.to_string()).await;

        match result {
            Ok(district) => web::Json(JsonResponse::<DistrictSchema, String>::ok(
                "Success to get district",
                Some(district),
            )),
            Err(error) => web::Json(JsonResponse::<DistrictSchema, String>::bad_request(
                "Failed to get district",
                Some(format!("Error: {}", error)),
            )),
        }
    }

    #[post("/api/v1/districts")]
    pub async fn create_district(
        state: web::Data<AppState>,
        payload: web::Json<CreateDistrictPayload>,
    ) -> impl Responder {
        let pool = state.get_pool().clone();
        let district = DistrictBuilder::new(
            Some(payload.get_name().to_string()),
            Some(payload.get_city_id().to_string()),
        );
        let result = DistrictModel::insert(&pool, &district.build()).await;

        println!("{:?}", result);

        match result {
            Ok(_) => web::Json(JsonResponse::<(), String>::ok(
                "Success to create district",
                None,
            )),
            Err(_) => web::Json(JsonResponse::<(), String>::bad_request(
                "Failed to create district",
                None,
            )),
        }
    }

    #[put("/api/v1/districts/{id}")]
    pub async fn update_district(
        state: web::Data<AppState>,
        id: web::Path<String>,
        payload: web::Json<CreateDistrictPayload>,
    ) -> impl Responder {
        let pool = state.get_pool().clone();
        let result = DistrictModel::update(&pool, &id.to_string(), &payload.clone()).await;

        match result {
            Ok(_) => web::Json(JsonResponse::<(), String>::ok(
                "Success to update district",
                None,
            )),
            Err(_) => web::Json(JsonResponse::<(), String>::bad_request(
                "Failed to update district",
                None,
            )),
        }
    }

    #[delete("/api/v1/districts/{id}")]
    pub async fn delete_district(
        state: web::Data<AppState>,
        id: web::Path<String>,
    ) -> impl Responder {
        let pool = state.get_pool().clone();
        let result = DistrictModel::delete(&pool, &id.to_string()).await;

        match result {
            Ok(_) => web::Json(JsonResponse::<(), String>::ok(
                "Success to delete district",
                None,
            )),
            Err(_) => web::Json(JsonResponse::<(), String>::bad_request(
                "Failed to delete district",
                None,
            )),
        }
    }
}
