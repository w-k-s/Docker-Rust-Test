#[macro_use] extern crate nickel;
#[macro_use] extern crate mysql;
extern crate mustache;
extern crate rustc_serialize;

use std::path::Path;
use std::collections::HashMap;
use mustache::{Data,MapBuilder};
use nickel::{Nickel, HttpRouter, StaticFilesHandler, Mount, Request, Response, MiddlewareResult,FormBody};

use mysql::{Pool,QueryResult};
use mysql::error::{Error,MySqlError};

#[derive(Debug, PartialEq, Eq)]
struct User{
    id: i32,
    username: String,
    password: String,
    first_name: String,
    last_name: String,
    email: String,
    is_active: bool,
    is_staff: bool,
}

struct AppConfig{
    db_string : String,
    host: String,
    port: String,
}

impl AppConfig{
    pub fn listen_string(&self) -> String{
       format!("{}:{}",self.host,self.port)
    }
}

fn main() {
    let config = AppConfig{
        db_string: "mysql://root:password@localhost:3306/surveys".to_owned(),
        host: "127.0.0.1".to_owned(),
        port: "8080".to_owned(),
    };
    let listen_addr = config.listen_string();
    let mut server = Nickel::with_data(config);

    server.get("/", middleware! { |_, mut response|
    	let mut data : HashMap<String,String> = HashMap::new();
        //response.headers_mut().set_raw("Connection",vec![b"close".to_vec()]);
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
    let pool = match(Pool::new(&*app_config.db_string)){
        Ok(pool) => pool,
        Err(x) => {
            let data = ViewModel {
                has_error: true,
                error: format!("{}",x),
            };
            return res.render("templates/index.tpl", &data);
        }
    };

    let params = req.form_body().unwrap();
    let email = params.get("email").unwrap();
    let password = params.get("password").unwrap();
    let first_name = params.get("first_name").unwrap();
    let last_name = params.get("last_name").unwrap();

    let mut stmt = pool.prepare(r"INSERT INTO users (username,password,first_name,last_name,email,is_active,is_staff)VALUES(:username,:password,:first_name,:last_name,:email,:is_active,:is_staff)").unwrap();
    let qr = match stmt.execute(params!{
        "username" => email,
        "password" => password,
        "first_name" => first_name,
        "last_name" => last_name,
        "email" => email,
        "is_active" => 1,
        "is_staff" => 0,
    }) {
        Ok(qr) => qr,
        Err(err) => {
            let message = match(err){
                Error::MySqlError(my_err) => if my_err.code == 1062 { "Username already exists".to_owned() }else{ format!("{}",my_err) },
                _ => format!("{}",err),
            };

            let data = ViewModel{
                has_error: true,
                error: message,
            };
            return res.render("templates/index.tpl",&data);
        }
    };
    
    let message = if qr.affected_rows() == 1 { "Sign-up successful".to_owned() }else{ "WTF".to_owned() };

    let data = ViewModel {
        has_error: true,
        error: message,
    };
    
    return res.render("templates/index.tpl", &data);    
}