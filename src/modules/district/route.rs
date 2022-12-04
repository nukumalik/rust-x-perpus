use super::controller::DistrictController;
use actix_web::web;

pub fn district_route(cfg: &mut web::ServiceConfig) {
    cfg.service(DistrictController::get_districts)
        .service(DistrictController::get_district)
        .service(DistrictController::create_district)
        .service(DistrictController::update_district)
        .service(DistrictController::delete_district);
}
