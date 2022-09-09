
use actix_web::{
    error,
    
    web, Error, HttpResponse, Result
};
use actix_session::{ Session};

//use crate::view_models::login::Login;

pub async fn index(
    tmpl: web::Data<tera::Tera>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let check = crate::view_models::login::LoginCheck::new();
    let mut ctx = tera::Context::new();
    ctx.insert("check", &check);
    
    println!("searching for token below!");

    if let Some(l) = session.get::<crate::view_models::login::Login>("session")? {
        //log::info!("SESSION value: {count}");

        let s = tmpl.render("home.html", &tera::Context::new())
    .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    return Ok(HttpResponse::Ok().content_type("text/html").body(s));
    }
    println!("token not found!");
    let s = 
        tmpl.render("index.html",  &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn save_token(session: Session,params: web::Form<crate::view_models::login::Login>,tmpl: web::Data<tera::Tera>) -> Result<HttpResponse,Error> {
    
    let mut ctx = tera::Context::new();
    
    if params.logging_in == false { // logout
        let check = crate::view_models::login::LoginCheck::new();
        ctx.insert("check", &check);
        session.purge();
         let s = tmpl.render("index.html", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    return Ok(HttpResponse::Ok().content_type("text/html").body(s))
    } else {
        let check = params.set_error();

        if check.has_error() {
            ctx.insert("check", &check);
            let s = tmpl.render("index.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?;

            return Ok(HttpResponse::Ok().content_type("text/html").body(s))
        }

        let mut l = crate::view_models::login::Login::new();
        l.endpoint_url = params.endpoint_url.clone();
        l.ibm_api_key_id  = params.ibm_api_key_id.clone();
        l.ibm_service_instance_id = params.ibm_service_instance_id.clone();
        l.bucket = params.bucket.clone();
        session.insert("session", l)?;
        let s = tmpl.render("home.html", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    return Ok(HttpResponse::Ok().content_type("text/html").body(s))
    }
        
}


