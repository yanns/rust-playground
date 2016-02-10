use std::io::Read;

use hyper::Client;
use hyper::header::Connection;
use hyper::header::{Headers, Authorization, Basic};
use rustc_serialize::json;

pub fn retrieve_token(auth_url: &str, project_key: &str, client_id: &str , client_secret: &str) -> Option<String> {
	let client = Client::new();

	let mut auth_headers = Headers::new();
	auth_headers.set(
	   Authorization(
		   Basic {
			   username: client_id.to_owned(),
			   password: Some(client_secret.to_owned())
		   }
	   ),
	);

	let url = format!("{}/oauth/token?grant_type=client_credentials&scope=manage_project:{}", auth_url, project_key);
	let mut res = client.post(&url)
		.header(Connection::close())
		.headers(auth_headers)
		.send().unwrap();

	let mut body = String::new();
	res.read_to_string(&mut body).unwrap();
	
	//res.status, hyper::Ok
	println!("Response: {}", body);

	json::Json::from_str(&body).ok()
		.and_then(|body_json| body_json.find("access_token")
			.and_then(|a| a.as_string().map(|s| s.to_string())))
}