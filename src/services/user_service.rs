

use std::sync::Arc;

use mysql;
use mysql::{Pool,QueryResult};
use mysql::error::{Error,MySqlError};

use frank_jwt::{Header, Payload, Algorithm, encode, decode};

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

	pub fn login(&self, username: &str, password: &str)->Result<User,String>{
		let mut stmt = self.pool.prepare(r"SELECT id,username,first_name,last_name,email,is_active,is_staff FROM users WHERE username = :username AND password = :password").unwrap();
		let mut qr = try!(stmt.execute(params!{
			"username" => username,
			"password" => password,
		}).map_err(|err|{
			return format!("{:?}",err);
		}));
	
		if let Some(row_result) = qr.next(){
			let (id,username,first_name,last_name,email,is_active,is_staff) = mysql::from_row(row_result.unwrap());
			let mut user = User{
					id: id,
					username: username,
					password: None,
					first_name: first_name,
					last_name: last_name,
					email: email,
					is_active: is_active,
					is_staff: is_staff,
					token: None,
			};
			try!(self.generate_token(&mut user));
			Ok(user)
		}else{
			Err("Username or password is incorrect".to_owned())
		}
	}

	fn generate_token(&self, user: &mut User)->Result<bool,String>{
		let mut payload = Payload::new();
		payload.insert("uid".to_string(), user.id.to_string());
		let header = Header::new(Algorithm::HS256);
		let secret = "secret123";

		user.token = Some(encode(header, secret.to_string(), payload.clone()));
		let mut stmt = self.pool.prepare(r"UPDATE users SET token = :token WHERE id = :id").unwrap();
		let mut qr = try!(stmt.execute(params!{
			"token" => user.token.clone(),
			"id" => user.id,
		}).map_err(|err|{
			return format!("{:?}",err);
		}));

		Ok(qr.affected_rows() == 1)
	}
}