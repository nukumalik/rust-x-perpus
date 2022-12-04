use super::controller::PublisherController;
use actix_web::web;

pub fn publisher_route(cfg: &mut web::ServiceConfig) {
    cfg.service(PublisherController::get_publishers)
        .service(PublisherController::get_publisher)
        .service(PublisherController::create_publisher)
        .service(PublisherController::update_publisher)
        .service(PublisherController::delete_publisher);
}
