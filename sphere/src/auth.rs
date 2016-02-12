use std::io::Read;

use hyper::Client;
use hyper::header::{Connection, Headers, Authorization, Basic};
use hyper::status::StatusCode;
use rustc_serialize::json;

pub fn retrieve_token(auth_url: &str, project_key: &str, client_id: &str , client_secret: &str) -> Result<String, String> {
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
	match client.post(&url)
		.header(Connection::close())
		.headers(auth_headers)
		.send() {

		Ok(ref mut res) if res.status!= StatusCode::Ok => {
			let mut body = String::new();
			let read_body = match res.read_to_string(&mut body) {
				Ok(_) => format!("Body: {}", body),
				Err(_) => "".to_string(),
			};
			Err(format!("request to '{}' delivers status {}. {}", url, res.status, read_body).to_owned())
		},
		Ok(mut res) => {
			let mut body = String::new();
			res.read_to_string(&mut body)
				.map_err(|err| err.to_string())
				.and_then(|_| {
					println!("Response: {}", body);
					json::Json::from_str(&body).map_err(|err| err.to_string())
						.and_then(|body_json| {
							body_json
								.find("access_token")
								.ok_or(format!("cannot find 'access_token' in json response body: {}", body_json).to_owned())
								.and_then(|a| {
									a.as_string()
										.map(|s| s.to_string())
										.ok_or(format!("access_token '{}' is not a string", a).to_owned())
								})
						})
				})
			},
		Err(err) => Err(err.to_string()),
	}
}