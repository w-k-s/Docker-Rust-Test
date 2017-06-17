mod services;
mod models;
#[macro_use] extern crate mysql;
#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate frank_jwt;

use std::sync::Arc;
use std::collections::HashMap;
use nickel::status::StatusCode;
use nickel::{Nickel, JsonBody, HttpRouter, StaticFilesHandler, Mount, Request, Response,Responder, MiddlewareResult};
use mysql::{Pool};
use rustc_serialize::json::ToJson;

use models::user::{Credentials, User};
use models::response::{ApiResult};

use services::user_service::{UserService};



struct AppConfig{
    pool : Arc<Pool>,
    host: String,
    port: String,
}

impl AppConfig{
    pub fn listen_string(&self) -> String{
       format!("{}:{}",self.host,self.port)
    }
}

fn main() {

    let pool = Pool::new("mysql://root:password@localhost:3306/surveys").map_err(|err|{
        panic!(format!("{}",err));
    }).unwrap();

    let config = AppConfig{
        pool: Arc::new(pool),
        host: "127.0.0.1".to_owned(),
        port: "8080".to_owned(),
    };
    let listen_addr = config.listen_string();
    let mut server = Nickel::with_data(config);

    //server.post("/api/register",register);
    server.post("/api/login",login);
   

	server.utilize(Mount::new("/",StaticFilesHandler::new("static/")));
    let _ = server.listen(listen_addr);

}

fn login<'a>(req: &mut Request<AppConfig>, mut res: Response<'a,AppConfig>) -> MiddlewareResult<'a, AppConfig> {

    let app_config =  req.server_data();
    let creds = try_with!(res, {
            req.json_as::<Credentials>().map_err(|e| (StatusCode::BadRequest, e))
        });

    let user_service = UserService::new(app_config.pool.clone());
    //let result : ApiResult<User> = try!(user_service.login(&creds.username,&creds.password)
    //.map(|user| ApiResult::Ok(StatusCode::Ok,user))
    //.map_err(|e| ApiResult::Err(StatusCode::BadRequest, format!("{}",e))));
    
    let fer : ApiResult<User> = match user_service.login(&creds.username,&creds.password){
        Ok(user) => ApiResult{data: Some(user), error: None},
        Err(err) => ApiResult{data: None, error: Some(format!("{}",err))},
    };

    return res.send(fer);
}

/*fn register<'a>(req: &mut Request<AppConfig>, mut res: Response<'a,AppConfig>) -> MiddlewareResult<'a, AppConfig> {
    //close connection after post request
    res.headers_mut().set_raw("Connection",vec![b"close".to_vec()]);

    #[derive(RustcEncodable)]
    struct ViewModel {
        has_error: bool,
        error: String,
    }

    let app_config =  req.server_data();
    let params = req.form_body().unwrap();

    let user = match get(params,vec!["email","password","first_name","last_name"]){
        Ok(values) => User{
            id: 0,
            username: (*values.get("email").unwrap()).to_owned(), 
            email: (*values.get("email").unwrap()).to_owned(),
            password: Some((*values.get("password").unwrap()).to_owned()),
            first_name: (*values.get("first_name").unwrap()).to_owned(),
            last_name: (*values.get("last_name").unwrap()).to_owned(),
            is_active: true,
            is_staff: false,
            token: None,
        },
        Err(missing) =>{
            let data = ViewModel{
                has_error: true,
                error: format!("Missing: {:?}",missing),
            };
            return res.render("templates/index.tpl", &data);
        }
    };

    let user_service = UserService::new(app_config.pool.clone());
    let success = match user_service.register(&user){
        Ok(success) => success,
        Err(err) => {
            let data = ViewModel{
                has_error: true,
                error: format!("{}",err),
            };
            return res.render("templates/index.tpl", &data);
        }
    };


    let data = ViewModel {
        has_error: true,
        error: if success { "Registration Successful ".to_owned() }else{ "Registration Failed".to_owned() },
    };
    
    return res.render("templates/index.tpl", &data);    
}

fn get<'a>(p : &'a Params, keys: Vec<&'a str>)->Result<HashMap<&'a str,&'a str>,Vec<&'a str>>{
    
    let mut provided_values : HashMap<&str,&str> = HashMap::new();
    let mut missing_keys = vec![];
    let mut missing : bool = false;
    for key in keys {
        match p.get(key){
            Some(value) if value.len() > 0 =>{ 
                provided_values.insert(key,value);
            },
            _ => {
                missing = true;
                missing_keys.push(key);
            }
        };
    }

    if missing { Err(missing_keys) }else{ Ok(provided_values) }
}*/