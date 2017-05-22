#[macro_use] extern crate nickel;
use std::path::Path;
use std::collections::HashMap;
use nickel::{Nickel, HttpRouter, StaticFilesHandler, Mount};

fn main() {
    let mut server = Nickel::new();

    server.get("/", middleware! { |_, response|
    	let mut data : HashMap<String,String> = HashMap::new();
    	return response.render("templates/index.tpl", &data);
    });

	server.utilize(Mount::new("/static/",StaticFilesHandler::new("static/")));
    server.listen("127.0.0.1:8080");
}

