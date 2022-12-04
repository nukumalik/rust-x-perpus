#[allow(non_snake_case)]
pub mod PublisherController {
    use super::super::{
        builder::PublisherBuilder, entity::CreatePublisherPayload, model::PublisherModel,
        schema::PublisherSchema,
    };
    use crate::{
        config::app_state::AppState, traits::builder::Builder, utils::json_response::JsonResponse,
    };
    use actix_web::{delete, get, post, put, web, Responder};

    #[get("/api/v1/publishers")]
    pub async fn get_publishers(state: web::Data<AppState>) -> impl Responder {
        let pool = state.get_pool().clone();
        let result = PublisherModel::select(&pool, None, None).await;

        match result {
            Ok(publishers) => web::Json(JsonResponse::<Vec<PublisherSchema>, String>::ok(
                "Success to get publishers",
                Some(publishers),
            )),
            Err(error) => web::Json(JsonResponse::<Vec<PublisherSchema>, String>::bad_request(
                "Failed to get publishers",
                Some(format!("Error: {}", error)),
            )),
        }
    }

    #[get("/api/v1/publishers/{id}")]
    pub async fn get_publisher(
        state: web::Data<AppState>,
        id: web::Path<String>,
    ) -> impl Responder {
        let pool = state.get_pool().clone();
        let result = PublisherModel::detail(&pool, &id.to_string()).await;

        match result {
            Ok(publisher) => web::Json(JsonResponse::<PublisherSchema, String>::ok(
                "Success to get publisher",
                Some(publisher),
            )),
            Err(error) => web::Json(JsonResponse::<PublisherSchema, String>::bad_request(
                "Failed to get publisher",
                Some(format!("Error: {}", error)),
            )),
        }
    }

    #[post("/api/v1/publishers")]
    pub async fn create_publisher(
        state: web::Data<AppState>,
        payload: web::Json<CreatePublisherPayload>,
    ) -> impl Responder {
        let pool = state.get_pool().clone();
        let publisher = PublisherBuilder::new(
            Some(payload.get_name().to_string()),
            Some(payload.get_country_id().to_string()),
            Some(payload.get_province_id().to_string()),
            Some(payload.get_city_id().to_string()),
            Some(payload.get_district_id().to_string()),
            Some(payload.get_street().to_string()),
            Some(payload.get_zip_code().to_string()),
        );
        let result = PublisherModel::insert(&pool, &publisher.build()).await;

        println!("{:?}", result);

        match result {
            Ok(_) => web::Json(JsonResponse::<(), String>::ok(
                "Success to create publisher",
                None,
            )),
            Err(_) => web::Json(JsonResponse::<(), String>::bad_request(
                "Failed to create publisher",
                None,
            )),
        }
    }

    #[put("/api/v1/publishers/{id}")]
    pub async fn update_publisher(
        state: web::Data<AppState>,
        id: web::Path<String>,
        payload: web::Json<CreatePublisherPayload>,
    ) -> impl Responder {
        let pool = state.get_pool().clone();
        let result = PublisherModel::update(&pool, &id.to_string(), &payload.clone()).await;

        match result {
            Ok(_) => web::Json(JsonResponse::<(), String>::ok(
                "Success to update publisher",
                None,
            )),
            Err(_) => web::Json(JsonResponse::<(), String>::bad_request(
                "Failed to update publisher",
                None,
            )),
        }
    }

    #[delete("/api/v1/publishers/{id}")]
    pub async fn delete_publisher(
        state: web::Data<AppState>,
        id: web::Path<String>,
    ) -> impl Responder {
        let pool = state.get_pool().clone();
        let result = PublisherModel::delete(&pool, &id.to_string()).await;

        match result {
            Ok(_) => web::Json(JsonResponse::<(), String>::ok(
                "Success to delete publisher",
                None,
            )),
            Err(_) => web::Json(JsonResponse::<(), String>::bad_request(
                "Failed to delete publisher",
                None,
            )),
        }
    }
}
