use super::controller::CityController;
use actix_web::web;

pub fn city_route(cfg: &mut web::ServiceConfig) {
    cfg.service(CityController::get_cities)
        .service(CityController::get_city)
        .service(CityController::create_city)
        .service(CityController::update_city)
        .service(CityController::delete_city);
}
