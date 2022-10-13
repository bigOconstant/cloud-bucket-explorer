use actix_web::{web};

use crate::template_logic::index::{index,save_bucket,add_bucket,delete_bucket};
use crate::template_logic::dashboard::{dashboard,details};


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("")
    .service(
        web::resource("/deletebucket")
                .route(web::get().to(delete_bucket))
    )
    .service(
web::resource("/dashboard")
        .route(web::get().to(dashboard))
    )
    
    .service(
        web::resource("/addbucket")  
        .route(web::get().to(add_bucket))
        .route(web::post().to(save_bucket))
    )
    .service(
        web::resource("/details")
                .route(web::get().to(details))
            )
    .service(
        web::resource("/")
        .route(web::get().to(index))
        .route(web::post().to(save_bucket)
    )));
}