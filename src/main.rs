mod services;
mod models;
#[macro_use] extern crate nickel;
#[macro_use] extern crate mysql;

extern crate mustache;
extern crate rustc_serialize;

use std::sync::Arc;
use std::path::Path;
use std::collections::HashMap;
use mustache::{Data,MapBuilder};
use nickel::{Nickel, HttpRouter, StaticFilesHandler, Mount, Request, Response, MiddlewareResult,FormBody};
use mysql::{Pool};

use models::user::{User};

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

    server.get("/", middleware! { |_, mut response|
    	let mut data : HashMap<String,String> = HashMap::new();
    	return response.render("templates/index.tpl", &data);
    });

    server.post("/register",register);

	server.utilize(Mount::new("/static/",StaticFilesHandler::new("static/")));
    server.listen(listen_addr);
}

fn register<'a>(req: &mut Request<AppConfig>, mut res: Response<'a,AppConfig>) -> MiddlewareResult<'a, AppConfig> {
    //close connection after post request
    res.headers_mut().set_raw("Connection",vec![b"close".to_vec()]);

    #[derive(RustcEncodable)]
    struct ViewModel {
        has_error: bool,
        error: String,
    }

    let app_config =  req.server_data();
    let params = req.form_body().unwrap();

    let user = User{
        id: 0,
        username: params.get("email").unwrap().to_owned(), 
        email: params.get("email").unwrap().to_owned(),
        password: params.get("password").unwrap().to_owned(),
        first_name: params.get("first_name").unwrap().to_owned(),
        last_name: params.get("last_name").unwrap().to_owned(),
        is_active: true,
        is_staff: false,
    };
    
    let user_service = UserService::new(app_config.pool.clone());
    let success = match user_service.register(&user){
        Ok(success) => success,
        Err(message) => {
            let data = ViewModel{
                has_error: true,
                error: message,
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