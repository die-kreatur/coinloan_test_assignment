use std::env;
use dotenv;

mod binance_client;
mod get_prices;
mod db_model;
mod db_methods;
mod schema;

#[macro_use]
extern crate diesel;

// use diesel::prelude::*;
// use diesel::pg::PgConnection;


#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();

	use crate::binance_client::Binance;
	use crate::get_prices::fetch_tickers;

	let acc = Binance {
		api_key: env::var("KEY").unwrap(),
		api_secret: env::var("SECRET").unwrap()
	};

	// let resp = fetch_tickers();
	// println!("{:#?}", resp);

	// let resp = acc.send_limit_order("blabla", "bla", "GTC", "bla", "bla").await;
    // println!("Data: {}", resp);
}
