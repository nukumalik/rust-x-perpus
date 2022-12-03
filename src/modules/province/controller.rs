#[allow(non_snake_case)]
pub mod ProvinceController {
    use super::super::{
        builder::ProvinceBuilder, entity::CreateProvincePayload, model::ProvinceModel,
        schema::ProvinceSchema,
    };
    use crate::{
        config::app_state::AppState, traits::builder::Builder, utils::json_response::JsonResponse,
    };
    use actix_web::{delete, get, post, put, web, Responder};

    #[get("/api/v1/provinces")]
    pub async fn get_provinces(state: web::Data<AppState>) -> impl Responder {
        let pool = state.get_pool().clone();
        let result = ProvinceModel::select(&pool, None, None).await;

        match result {
            Ok(provinces) => web::Json(JsonResponse::<Vec<ProvinceSchema>, String>::ok(
                "Success to get provinces",
                Some(provinces),
            )),
            Err(error) => web::Json(JsonResponse::<Vec<ProvinceSchema>, String>::bad_request(
                "Failed to get provinces",
                Some(format!("Error: {}", error)),
            )),
        }
    }

    #[get("/api/v1/provinces/{id}")]
    pub async fn get_province(state: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
        let pool = state.get_pool().clone();
        let result = ProvinceModel::detail(&pool, &id.to_string()).await;

        match result {
            Ok(province) => web::Json(JsonResponse::<ProvinceSchema, String>::ok(
                "Success to get province",
                Some(province),
            )),
            Err(error) => web::Json(JsonResponse::<ProvinceSchema, String>::bad_request(
                "Failed to get province",
                Some(format!("Error: {}", error)),
            )),
        }
    }

    #[post("/api/v1/provinces")]
    pub async fn create_province(
        state: web::Data<AppState>,
        payload: web::Json<CreateProvincePayload>,
    ) -> impl Responder {
        let pool = state.get_pool().clone();
        let province = ProvinceBuilder::new(
            Some(payload.get_name().to_string()),
            Some(payload.get_country_id().to_string()),
        );
        let result = ProvinceModel::insert(&pool, &province.build()).await;

        println!("{:?}", result);

        match result {
            Ok(_) => web::Json(JsonResponse::<(), String>::ok(
                "Success to create province",
                None,
            )),
            Err(_) => web::Json(JsonResponse::<(), String>::bad_request(
                "Failed to create province",
                None,
            )),
        }
    }

    #[put("/api/v1/provinces/{id}")]
    pub async fn update_province(
        state: web::Data<AppState>,
        id: web::Path<String>,
        payload: web::Json<CreateProvincePayload>,
    ) -> impl Responder {
        let pool = state.get_pool().clone();
        let result = ProvinceModel::update(&pool, &id.to_string(), &payload.clone()).await;

        match result {
            Ok(_) => web::Json(JsonResponse::<(), String>::ok(
                "Success to update province",
                None,
            )),
            Err(_) => web::Json(JsonResponse::<(), String>::bad_request(
                "Failed to update province",
                None,
            )),
        }
    }

    #[delete("/api/v1/provinces/{id}")]
    pub async fn delete_province(
        state: web::Data<AppState>,
        id: web::Path<String>,
    ) -> impl Responder {
        let pool = state.get_pool().clone();
        let result = ProvinceModel::delete(&pool, &id.to_string()).await;

        match result {
            Ok(_) => web::Json(JsonResponse::<(), String>::ok(
                "Success to delete province",
                None,
            )),
            Err(_) => web::Json(JsonResponse::<(), String>::bad_request(
                "Failed to delete province",
                None,
            )),
        }
    }
}
