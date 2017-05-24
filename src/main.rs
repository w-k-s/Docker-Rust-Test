#[macro_use] extern crate nickel;
extern crate mustache;
extern crate rustc_serialize;

use std::path::Path;
use std::collections::HashMap;
use mustache::{Data,MapBuilder};
use nickel::{Nickel, HttpRouter, StaticFilesHandler, Mount, Request, Response, MiddlewareResult};

extern crate mysql;

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

struct AppConfiguration{
    db_string : String,
    host: String,
    port: String,
}

impl AppConfiguration{
    pub fn listen_string(&self) -> String{
       format!("{}:{}",self.host,self.port)
    }
}

fn main() {
    let config = AppConfiguration{
        db_string: "mysql://root:password@localhost:3307/surveys".to_owned(),
        host: "127.0.0.1".to_owned(),
        port: "8080".to_owned(),
    };

    let mut server = Nickel::new();

    server.get("/", middleware! { |_, mut response|
    	let mut data : HashMap<String,String> = HashMap::new();
        //response.headers_mut().set_raw("Connection",vec![b"close".to_vec()]);
    	return response.render("templates/index.tpl", &data);
    });
    server.post("/register",register);

	server.utilize(Mount::new("/static/",StaticFilesHandler::new("static/")));
    server.listen(config.listen_string());
}

fn register<'a>(_: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
    //connect to database
    /*let conn = Connection::connect("postgres://root:root@localhost/json-test", &SslMode::None).unwrap();
    let blog = Blog {
        id: 0,
        content: "My second blogpost".to_string(),
        author: "Mike".to_string(),
        datepost: "".to_string()
    };
    // insert data in DB
    conn.execute("INSERT INTO blogs (content, author) VALUES ($1, $2)",
    &[&blog.content, &blog.author]).unwrap();
    */
    #[derive(RustcEncodable)]
    struct ViewModel {
        has_error: bool,
        error: &'static str,
    }

    let data = ViewModel {
        has_error: true,
        error: "Registration is unavailable" 
    };
    res.headers_mut().set_raw("Connection",vec![b"close".to_vec()]);
    return res.render("templates/index.tpl", &data);
}