extern crate hyper;
extern crate rustc_serialize;
extern crate clap;
extern crate chrono;
#[macro_use] extern crate log;
extern crate env_logger;

use std::io::Read;

use clap::App;
use hyper::Client;
use hyper::header::Connection;
use hyper::header::{Headers, Authorization, Bearer};

mod auth;

fn main() {
	env_logger::init().unwrap();
	let matches = App::new("sphere")
		.version("1.0")
		.author("Yann Simon <yann.simon.fr@gmail.com>")
		.args_from_usage(
			"<PROJECT_KEY> 'project key'
			 <CLIENT_ID> 'client ID'
			 <CLIENT_SECRET> 'client secret'
			 --auth_url=[AUTH_URL] 'authentication URL (default to \"https://auth.sphere.io\")'
			 --api_url=[API_URL] 'api URL (default to \"https://api.sphere.io\")'")
		.get_matches();

	let project_key = matches.value_of("PROJECT_KEY").unwrap();
	let client_id = matches.value_of("CLIENT_ID").unwrap();
	let client_secret = matches.value_of("CLIENT_SECRET").unwrap();
	let auth_url = matches.value_of("AUTH_URL").unwrap_or("https://auth.sphere.io");
	let api_url = matches.value_of("API_URL").unwrap_or("https://api.sphere.io");


	match auth::retrieve_token(auth_url, project_key, client_id, client_secret) {
		Err(err) => panic!("error: {}", err),
		Ok(token) => {
			let access_token = token.access_token();
			debug!("token: {} {}", access_token, token.is_valid());

			let mut headers = Headers::new();
			headers.set(
				Authorization(
					Bearer {
						token: access_token.to_owned()
					}
				)
			);

			let client = Client::new();

			let uri = format!("{}/{}/products?limit=1", api_url, project_key);
			let mut projets_res = client.get(&uri)
				.header(Connection::close())
				.headers(headers)
				.send()
				.unwrap();

			let mut body = String::new();
			projets_res.read_to_string(&mut body).unwrap();

			println!("Response: {}", body);
		}
	}
}