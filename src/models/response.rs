extern crate nickel;
extern crate rustc_serialize;

// Nickel
use nickel::{Response, Responder, MiddlewareResult, MediaType};
use nickel::status::StatusCode::{self};

// rustc_serialize
use rustc_serialize::json::{Json, ToJson};

use std::collections::BTreeMap;

pub struct ApiResult<T: ToJson>{
	pub data : Option<T>,
	pub error: Option<String>
}

impl <T: ToJson> ToJson for ApiResult<T>{

	fn to_json(&self)->Json{
		let mut d = BTreeMap::new();
		if self.data.is_some(){
			 d.insert("data".to_string(),self.data.to_json());
		}
		if self.error.is_some(){
			d.insert("error".to_string(),self.error.to_json());
		}
		
        Json::Object(d)
	}
}

impl<D,T> Responder<D> for ApiResult<T> where T: ToJson {

    fn respond<'a>(self, mut response: Response<'a, D>) -> MiddlewareResult<'a, D> {
        response.set(MediaType::Json);
        response.send(self.to_json())
    }
}
