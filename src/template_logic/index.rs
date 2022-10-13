
use std::collections::HashMap;

use actix_web::{
    error,
    
    web, Error, HttpResponse, Result
};
use actix_session::{ Session};
use crate::view_models::login::LoginArray;
//use crate::view_models::login::Login;

pub async fn index(
    tmpl: web::Data<tera::Tera>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let check = crate::view_models::login::LoginCheck::new();
    let mut ctx = tera::Context::new();
    ctx.insert("check", &check);
    
    println!("In index!");
    if let Some(l) = session.get::<LoginArray>("session")? {
        //let login_array:LoginArray = LoginArray{buckets:vec![l]};
        println!("Found login array inserting objects");

        ctx.insert("objects", &l);

        let s = tmpl.render("home.html", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    return Ok(HttpResponse::Ok().content_type("text/html").body(s));
    }
    println!("end index!");
    
    let s = 
        tmpl.render("index.html",  &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn save_bucket(session: Session,
    params: web::Form<crate::view_models::login::Login>,
    tmpl: web::Data<tera::Tera>) -> Result<HttpResponse,Error> {

    let mut la:LoginArray = LoginArray{buckets:vec![]};
    if let Some(l) = session.get::<LoginArray>("session")? {
        la = l;
    }
    
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
        la.buckets.append(&mut vec![l.clone()]);
        session.insert("session", la.clone())?;
        ctx.insert("objects", &la);
        let s = tmpl.render("home.html", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    return Ok(HttpResponse::Ok().content_type("text/html").body(s))
    }
        
}

pub async fn add_bucket(
    tmpl: web::Data<tera::Tera>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let check = crate::view_models::login::LoginCheck::new();
    let mut ctx = tera::Context::new();
    ctx.insert("check", &check);
    
    let s = 
        tmpl.render("index.html",  &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn delete_bucket(session: Session,
    query: web::Query<HashMap<String, String>>,
    tmpl: web::Data<tera::Tera>) -> Result<HttpResponse,Error> {
    println!("inside delete bucket!");
    let mut ctx = tera::Context::new();
    
    if let Some(mut l) = session.get::<LoginArray>("session")? {
        let mut index:i32 = 0;
        if let Some(index_inside) = query.get("index") {
            println!("index found");
            let temp_value = index_inside.parse::<i32>();
            match temp_value {
                Ok(x) => {
                    index = x;
                    l.buckets.remove(index as usize);
                },
                Err(e) => {
                    let s = tmpl.render("error_generic.html", &ctx)
                    .map_err(|_| error::ErrorInternalServerError("Template error"))?;
                    return Ok(HttpResponse::Ok().content_type("text/html").body(s))
                },
            }
            
        }else {
            let s = tmpl.render("error_generic.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?;
            return Ok(HttpResponse::Ok().content_type("text/html").body(s))
        }
    
        //session.purge();
        session.insert("session", l.clone())?;
    
        let s = tmpl.render("bucket_removed.html", &ctx)
         .map_err(|_| error::ErrorInternalServerError("Template error"))?;
         return Ok(HttpResponse::Ok().content_type("text/html").body(s))
    }
    let s = tmpl.render("error_generic.html", &ctx)
                    .map_err(|_| error::ErrorInternalServerError("Template error"))?;
                    return Ok(HttpResponse::Ok().content_type("text/html").body(s))
    
}



// pub async fn save_bucket(session: Session,
//     params: web::Form<crate::view_models::login::Login>,
//     tmpl: web::Data<tera::Tera>) -> Result<HttpResponse,Error> {

//     let mut la:LoginArray = LoginArray{buckets:vec![]};
//     if let Some(l) = session.get::<LoginArray>("session")? {
//         la = l;
//     }
    
//     let mut ctx = tera::Context::new();
    
//     if params.logging_in == false { // logout
//         let check = crate::view_models::login::LoginCheck::new();
//         ctx.insert("check", &check);
//         session.purge();
//          let s = tmpl.render("index.html", &ctx)
//     .map_err(|_| error::ErrorInternalServerError("Template error"))?;
//     return Ok(HttpResponse::Ok().content_type("text/html").body(s))
//     } else {
//         let check = params.set_error();

//         if check.has_error() {
//             ctx.insert("check", &check);
//             let s = tmpl.render("index.html", &ctx)
//             .map_err(|_| error::ErrorInternalServerError("Template error"))?;

//             return Ok(HttpResponse::Ok().content_type("text/html").body(s))
//         }

//         let mut l = crate::view_models::login::Login::new();
//         l.endpoint_url = params.endpoint_url.clone();
//         l.ibm_api_key_id  = params.ibm_api_key_id.clone();
//         l.ibm_service_instance_id = params.ibm_service_instance_id.clone();
//         l.bucket = params.bucket.clone();
//         la.buckets.append(&mut vec![l.clone()]);
//         session.insert("session", la.clone())?;
//         ctx.insert("objects", &la);
//         let s = tmpl.render("home.html", &ctx)
//     .map_err(|_| error::ErrorInternalServerError("Template error"))?;
//     return Ok(HttpResponse::Ok().content_type("text/html").body(s))
//     }
        
// }


