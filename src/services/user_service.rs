use std::sync::Arc;

use mysql::{Pool,QueryResult};
use mysql::error::{Error,MySqlError};

use models::user::{User};

pub struct UserService{
	pool : Arc<Pool>
}

impl UserService{

	pub fn new(pool : Arc<Pool>)->UserService{
		UserService{
			pool: pool,
		}
	}

	pub fn register(&self, user: &User)->Result<bool,String>{
		let mut stmt = self.pool.prepare(r"INSERT INTO users (username,password,first_name,last_name,email,is_active,is_staff)VALUES(:username,:password,:first_name,:last_name,:email,:is_active,:is_staff)").unwrap();
	    let qr = stmt.execute(params!{
	        "username" => user.email.clone(),
	        "password" => user.password.clone(),
	        "first_name" => user.first_name.clone(),
	        "last_name" => user.last_name.clone(),
	        "email" => user.email.clone(),
	        "is_active" => user.is_active,
	        "is_staff" => user.is_staff,
	    }).map_err(|err|{
	    	
	    	let message = match err {
                Error::MySqlError(my_err) => if my_err.code == 1062 { "Username already exists".to_owned() }else{ format!("{}",my_err) },
                _ => format!("{}",err),
            };

            return message;
	    }).unwrap();

	    return Ok(qr.affected_rows() == 1 )
	}

	/*pub fn login(&self, username: &str, password: &str)->Result<User,String>{
		let mut stmt = self.pool.prepare(r"SELECT * FROM users WHERE username = :username AND password = :password").unwrap();
		let qr = stmt.execute(params!{
			"username" => username,
			"password" => password,
		}).map(|err|){
			return format!("{}",err);
		};
	}*/
}