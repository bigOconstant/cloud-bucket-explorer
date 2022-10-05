use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize,)]
pub struct FieldError{
    pub valid:bool,
    pub error_message:String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct Login {
    pub ibm_api_key_id: String,
    pub ibm_service_instance_id: String,
    pub endpoint_url: String,
    pub logging_in: bool,
    pub bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct LoginArray {
    pub buckets: Vec<Login>,
}

#[derive(Serialize, Deserialize,)]
pub struct LoginCheck{
    pub ibm_api_key_id:FieldError,
    pub bucket:FieldError,
    pub ibm_service_instance_id:FieldError,
    pub endpoint_url:FieldError,
}

impl LoginCheck {
    pub fn new()->LoginCheck {
        return LoginCheck {bucket:FieldError{valid:true,error_message:"".to_string()}, ibm_api_key_id: FieldError{valid:true,error_message:"".to_string()}, ibm_service_instance_id: FieldError{valid:true,error_message:"".to_string()},endpoint_url: FieldError{valid:true,error_message:"".to_string()} };
    }  

    pub fn has_error(&self)->bool {
        if !self.ibm_api_key_id.valid || !self.ibm_service_instance_id.valid || !self.endpoint_url.valid {
            return true;
        }else {
            return false
        }
    }

    pub fn set_ibm_api_key_id_error(&mut self,err_message:&str) {
        self.ibm_api_key_id.error_message = err_message.to_string();
        self.ibm_api_key_id.valid = false;
    }

    pub fn set_ibm_service_instance_id_error(&mut self,err_message:&str) {
        self.ibm_service_instance_id.error_message = err_message.to_string();
        self.ibm_service_instance_id.valid = false;
    }

    pub fn set_endpoint_url_error(&mut self,err_message:&str) {
        self.endpoint_url.error_message = err_message.to_string();
        self.endpoint_url.valid = false;
    }
}

impl Login {
    pub fn new()->Login {
        return Login { bucket:"".to_string(), ibm_api_key_id: "".to_string(), ibm_service_instance_id: "".to_string(),endpoint_url: "".to_string(),logging_in:true};
    }

 
    
    pub fn set_error(&self)->LoginCheck {
        let mut ret_val = LoginCheck::new();
        if self.ibm_api_key_id.is_empty(){
            ret_val.ibm_api_key_id.valid = false;
            ret_val.ibm_api_key_id.error_message = "ibm_api_key_id required".to_string();
        } 

        if self.ibm_service_instance_id.is_empty(){
            ret_val.ibm_service_instance_id.valid = false;
            ret_val.ibm_service_instance_id.error_message = "ibm_service_instance_id required".to_string();
        } 

        if self.endpoint_url.is_empty(){
            ret_val.endpoint_url.valid = false;
            ret_val.endpoint_url.error_message = "endpoint_url required".to_string();
        } 

        return ret_val;

    }

}