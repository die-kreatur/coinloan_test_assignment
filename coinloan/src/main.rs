use std::env;

mod binance_client;
mod get_prices;


#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();

	use binance_client::binance_client::Binance;
	use get_prices::get_prices::fetch_tickers;

	let acc = Binance {
		api_key: env::var("KEY").unwrap(),
		api_secret: env::var("SECRET").unwrap()
	};

	let resp = fetch_tickers();
	println!("{:#?}", resp);

	let resp = acc.send_limit_order("blabla", "bla", "GTC", "bla", "bla").await;
    println!("Data: {}", resp);
}
