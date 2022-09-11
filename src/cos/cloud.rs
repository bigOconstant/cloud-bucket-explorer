use serde_derive::Deserialize;
use serde_derive::Serialize;
use reqwest::{Error,};
use reqwest::header::USER_AGENT;
use std::collections::HashMap;
use serde_xml_rs::{from_str, to_string, de::Deserializer};
extern crate bytesize;
use bytesize::ByteSize;
use urlencoding::encode;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListBucketResult {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Marker")]
    pub marker: String,
    #[serde(rename = "MaxKeys")]
    pub max_keys: i64,
    #[serde(rename = "Delimiter")]
    pub delimiter: String,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "Contents")]
    pub contents: Vec<Content>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "LastModified")]
    pub last_modified: String,
    #[serde(rename = "ETag")]
    pub etag: String,
    #[serde(rename = "Size")]
    pub size: i64,
    #[serde(default)]
    pub size_label:String,
    #[serde(rename = "Owner")]
    pub owner: Owner,
    #[serde(rename = "StorageClass")]
    pub storage_class: String,
    #[serde(skip_deserializing)]
    pub url:String,
}

impl Content {
    fn set_url(&mut self) {
        let val :String = format!("/details?key={}&last_modified={}&size_label={}&owner={}",encode(&self.key),encode(&self.last_modified),encode(&self.size_label),encode(&self.owner.display_name));
        self.url = val;
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "DisplayName")]
    pub display_name: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
    #[serde(rename = "ims_user_id")]
    pub ims_user_id: i64,
    #[serde(rename = "token_type")]
    pub token_type: String,
    #[serde(rename = "expires_in")]
    pub expires_in: i64,
    pub expiration: i64,
    pub scope: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct Cloud {
    pub token: Token,
    pub objectList:ListBucketResult,
    pub credentials:crate::view_models::login::Login
}

impl Cloud {
    pub fn new( input: crate::view_models::login::Login  )->Cloud {
        let c:Cloud = Cloud {
            credentials:input.clone(),
            ..Default::default()
        };
        return c;
    }
    pub fn set_urls(&mut self) {
        for n in 0..self.objectList.contents.len() {
            self.objectList.contents[n].set_url();
        }
    }
    async fn getToken(&mut self) -> Result<(), Error> {
        let client = reqwest::Client::new();


        println!("getting token!");
    
        let mut params = HashMap::new();
        let response_type = "cloud_iam".to_string();
        let grant_type = "urn:ibm:params:oauth:grant-type:apikey".to_string();
        params.insert("apikey", &self.credentials.ibm_api_key_id);
        params.insert("response_type", &response_type);
        params.insert("grant_type", &grant_type);
    
    
        let response = client
            .post("https://iam.cloud.ibm.com/oidc/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await?;

         self.token = response.json().await?;
        
         
        Ok(())
    }

    pub async fn getObjects(&mut self, prefix: String) -> Result<(), Error> {
        self.getToken().await?;//Could check it this exist if not check if expired if not skip a call
        let client = reqwest::Client::new();
        let token_string = format!("{}{}", "bearer ".to_string(), self.token.access_token.clone());
        let url = format!("{}/{}?prefix={}",self.credentials.endpoint_url,self.credentials.bucket,prefix);
        let response = client
            .get(url)
            .header("Authorization", token_string)
            .send()
            .await?;

        let printing = response.text().await?;
        let objects:Result<ListBucketResult,serde_xml_rs::Error> = from_str(printing.as_str());
        //self.objectList = from_str(printing.as_str())?;
        match objects {
            Ok(obj) => {
                self.objectList = obj;
            },
            Err(e) => {
                println!("{}",e)
            },
        }
        
        for n in 0..self.objectList.contents.len() {
            self.objectList.contents[n].size_label =  ByteSize::b(self.objectList.contents[n].size.clone() as u64).to_string();
        }

        Ok(())
    }
}