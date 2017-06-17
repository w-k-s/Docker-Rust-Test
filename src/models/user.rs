use rustc_serialize::json::{self, ToJson, Json};
use std::collections::BTreeMap;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Credentials{
	pub username : String,
	pub password : String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct User{
    pub id: i32,
    pub username: String,
    pub password: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub is_active: bool,
    pub is_staff: bool,
    pub token: Option<String>,
}

impl ToJson for User{
	fn to_json(&self) -> Json {
        let mut map = BTreeMap::new();
        map.insert("id".to_string(),self.id.to_json());
        map.insert("first_name".to_string(),self.first_name.to_json());
        map.insert("last_name".to_string(),self.last_name.to_json());
        map.insert("email".to_string(),self.email.to_json());
        map.insert("is_active".to_string(),self.is_active.to_json());
        map.insert("is_staff".to_string(),self.is_staff.to_json());
        self.token.clone().map(|token| map.insert("token".to_string(),token.to_json()));
        Json::Object(map)
    }
}