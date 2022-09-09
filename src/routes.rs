use actix_web::{web};

use crate::template_logic::index::{index,save_token};
use crate::template_logic::dashboard::{dashboard};


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("")
    .service(
web::resource("/dashboard")
        .route(web::get().to(dashboard))
    )
    .service(
        web::resource("/")
        .route(web::get().to(index))
        .route(web::post().to(save_token)
    )));
}