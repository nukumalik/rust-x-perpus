use super::controller::CountryController;
use actix_web::web;

pub fn country_route(cfg: &mut web::ServiceConfig) {
    cfg.service(CountryController::get_countries)
        .service(CountryController::get_country)
        .service(CountryController::create_country)
        .service(CountryController::update_country)
        .service(CountryController::delete_country);
}
