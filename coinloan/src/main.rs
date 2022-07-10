use std::env;

mod binance_client;


#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();

	use binance_client::binance_client::Binance;

	let acc = Binance {
		api_key: env::var("KEY").unwrap(),
		api_secret: env::var("SECRET").unwrap()
	};

	let resp = acc.send_limit_order("blabla", "bla", "GTC", "bla", "bla").await;
    println!("Data: {}", resp);
}
