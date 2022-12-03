use super::controller::ProvinceController;
use actix_web::web;

pub fn province_route(cfg: &mut web::ServiceConfig) {
    cfg.service(ProvinceController::get_provinces)
        .service(ProvinceController::get_province)
        .service(ProvinceController::create_province)
        .service(ProvinceController::update_province)
        .service(ProvinceController::delete_province);
}
