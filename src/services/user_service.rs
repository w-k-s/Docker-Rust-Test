

use std::sync::Arc;

use mysql;
use mysql::{Pool,Params};
use mysql::error::{Error};

use frank_jwt::{Header, Payload, Algorithm, encode};

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
		
		let mut user = try!(self.find_user("username = :username",Params::from(params!{"username"=>username.to_owned()})));

		let verified_password = user.password.unwrap().clone();
		user.password = None;

		if verified_password == password{
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
		let qr = try!(stmt.execute(params!{
			"token" => user.token.clone(),
			"id" => user.id,
		}).map_err(|err|{
			return format!("{:?}",err);
		}));

		Ok(qr.affected_rows() == 1)
	}

	pub fn user_with_token(&self, token: &str)->Result<User,String>{
		self.find_user("token = :token",Params::from(params!{"token"=>token.to_owned()}))
	}

	fn find_user<>(&self, where_clause: &str, params : Params)->Result<User,String>{
		let query = format!(r"SELECT id,username,password, first_name,last_name,email,is_active,is_staff,token FROM users WHERE {}",where_clause);
		let mut stmt = try!(self.pool.prepare(query).map_err(|err|{
			return format!("{}",err)
		}));

		let mut qr = try!(stmt.execute(params).map_err(|err|{
			return format!("{:?}",err);
		}));
	
		if let Some(row_result) = qr.next(){
			let (id,username,password,first_name,last_name,email,is_active,is_staff,token) = mysql::from_row(row_result.unwrap());
			let user = User{
					id: id,
					username: username,
					password: password,
					first_name: first_name,
					last_name: last_name,
					email: email,
					is_active: is_active,
					is_staff: is_staff,
					token: token,
			};
			
			Ok(user)
		}else{
			Err("User not Found".to_owned())
		}
	}
}