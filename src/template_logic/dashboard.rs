use actix_web::{
    error,
    
    web, Error, HttpResponse, Result, HttpRequest
};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use actix_session::{ Session};
use crate::cos::cloud::{Cloud};
use crate::view_models::login::LoginArray;

pub async fn dashboard(
    tmpl: web::Data<tera::Tera>,
    session: Session,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    
    if let Some(l) = session.get::<LoginArray>("session")? {
        
        let mut prefix:String;
        let mut index:i32 = 0;
        if let Some(pref) = query.get("prefix") {
            prefix = pref.clone();
        }else {
            prefix = "".to_string();
        }
        if let Some(index_inside) = query.get("index") {
            println!("index found");
            let temp_value = index_inside.parse::<i32>();
            match temp_value{
                Ok(x) => {
                    index = x;
                },
                Err(e) => {
                    println!("Error on index value");
                },
            }
            
        }else {
            index = 0;
        }
        if l.buckets.len() <0 || l.buckets.len() < index as usize  {
            println!("incorrect index");
            let s = 
            tmpl.render("please_login.html",  &tera::Context::new())
                    .map_err(|_| error::ErrorInternalServerError("Template error"))?;
            
            return Ok(HttpResponse::Ok().content_type("text/html").body(s))
        } 
        let mut c:Cloud = Cloud::new(l.buckets[index as usize].clone());
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
    pub delete: String,
    #[serde(skip_deserializing)]
    pub cred:crate::view_models::login::Login,
}

impl ObjectDetails {
    pub fn new(l:crate::view_models::login::Login) -> ObjectDetails {
        ObjectDetails { cred:l.clone(),key: "".to_string(), last_modified: "".to_string(),size_label: "".to_string(), owner: "".to_string(),delete:"false".to_string() }
    }
}

pub async fn details(
    tmpl: web::Data<tera::Tera>,
    session: Session,
    req:HttpRequest,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let mut index:i32 = 0;

    if let Some(l) = session.get::<LoginArray>("session")? {
        if let Some(index) = query.get("index") {
            println!("index found");
        }else {
            index = 0;
        }

        if l.buckets.len() <0 || l.buckets.len() < index as usize  {
            println!("incorrect index");
            let s = 
            tmpl.render("please_login.html",  &tera::Context::new())
                    .map_err(|_| error::ErrorInternalServerError("Template error"))?;
            
            return Ok(HttpResponse::Ok().content_type("text/html").body(s))
        }


        let c:Cloud = Cloud::new(l.buckets[index as usize].clone());
        let mut obj = ObjectDetails::new(l.buckets[index as usize].clone());
        let object= web::Query::<ObjectDetails>::from_query(req.query_string());
        match object {
            Ok(o) => {
                obj.key = o.key.clone();
                obj.last_modified = o.last_modified.clone();
                obj.size_label = o.size_label.clone();
                obj.owner = o.owner.clone();
                obj.delete = o.delete.clone();
            },
            Err(_) => {
                println!("incorrect parameter format")
            },
        }
        let mut ctx = tera::Context::new();
        ctx.insert("object", &obj);
        let s:String;
        if obj.delete == "true".to_string() {
           let _ = details_delete(obj.key.clone(),c).await;
            
             s = tmpl.render("deleteobject.html", &ctx)
                .map_err(|e| error::ErrorInternalServerError(e))?;
                
        }else{
            s = tmpl.render("objectdetails.html", &ctx)
            .map_err(|e| error::ErrorInternalServerError(e))?;
        }
        return Ok(HttpResponse::Ok().content_type("text/html").body(s));

    }
    let s = 
        tmpl.render("please_login.html",  &tera::Context::new())
            .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn details_delete(
    item:String,
    mut c: Cloud
) -> Result<(), reqwest::Error> {
    println!("Calling delete!");
    let ret_val =   c.delete_objects(vec![item]).await;

    

    return ret_val;
}