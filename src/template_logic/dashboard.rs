use actix_web::{
    error,
    
    web, Error, HttpResponse, Result
};
use std::collections::HashMap;

use actix_session::{ Session};
use crate::cos::cloud::{Cloud};


pub async fn dashboard(
    tmpl: web::Data<tera::Tera>,
    session: Session,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    
    if let Some(l) = session.get::<crate::view_models::login::Login>("session")? {
        let mut c:Cloud = Cloud::new(l.clone());
        let mut prefix:String;
        if let Some(pref) = query.get("prefix") {
            prefix = pref.clone();
        }else {
            prefix = "".to_string();
        }
        let result = c.getObjects(prefix).await;
        match result {
            Ok(_) => {
                let mut ctx = tera::Context::new();
                ctx.insert("items", &c.objectList.contents);
                let s = tmpl.render("dashboard.html", &ctx)
                .map_err(|e| error::ErrorInternalServerError(e))?;
                return Ok(HttpResponse::Ok().content_type("text/html").body(s));
            }
            Err(_) => {
                c = Cloud::new(l);
                let mut ctx = tera::Context::new();
                ctx.insert("items", &c.objectList.contents);
                let s = tmpl.render("dashboard.html", &ctx)
                .map_err(|e| error::ErrorInternalServerError(e))?;
                return Ok(HttpResponse::Ok().content_type("text/html").body(s));
            },
        }
        
    }
    let s = 
        tmpl.render("please_login.html",  &tera::Context::new())
            .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}