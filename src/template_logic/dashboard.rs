use actix_web::{
    error,
    
    web, Error, HttpResponse, Result, HttpRequest
};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

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
                c.set_urls();
                ctx.insert("items", &c.objectList.contents);
                ctx.insert("size",&c.get_total_size());
                let s = tmpl.render("dashboard.html", &ctx)
                .map_err(|e| error::ErrorInternalServerError(e))?;
                return Ok(HttpResponse::Ok().content_type("text/html").body(s));
            }
            Err(e) => {
                print!("error getting results:{}",e);
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



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectDetails {
    pub key: String,
    pub last_modified: String,
    pub size_label:String,
    pub owner:String,
    #[serde(skip_deserializing)]
    pub cred:crate::view_models::login::Login,
}

impl ObjectDetails {
    pub fn new(l:crate::view_models::login::Login) -> ObjectDetails {
        ObjectDetails { cred:l.clone(),key: "".to_string(), last_modified: "".to_string(),size_label: "".to_string(), owner: "".to_string() }
    }
}

pub async fn details(
    tmpl: web::Data<tera::Tera>,
    session: Session,
    req:HttpRequest,
) -> Result<HttpResponse, Error> {
    
    if let Some(l) = session.get::<crate::view_models::login::Login>("session")? {

        let mut obj = ObjectDetails::new(l.clone());
        let object= web::Query::<ObjectDetails>::from_query(req.query_string());
        match object {
            Ok(o) => {
                obj.key = o.key.clone();
                obj.last_modified = o.last_modified.clone();
                obj.size_label = o.size_label.clone();
                obj.owner = o.owner.clone();
            },
            Err(_) => {
                println!("incorrect parameter format")
            },
        }
        
        let mut ctx = tera::Context::new();
        ctx.insert("object", &obj);
        let s = tmpl.render("objectdetails.html", &ctx)
                .map_err(|e| error::ErrorInternalServerError(e))?;
                return Ok(HttpResponse::Ok().content_type("text/html").body(s));

    }
    let s = 
        tmpl.render("please_login.html",  &tera::Context::new())
            .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}