use crate::db_methods::{establish_connection, list_orders, update_order};
use bigdecimal::ToPrimitive;
use crate::get_prices::fetch_tickers;
use std::env;
use reqwest::blocking::Client;
use crate::binance_client::Binance;


fn get_params_for_limit_order() -> (i32, String, String, f64, f64) {
    let conn = establish_connection();

    let last_order = list_orders(&conn);
    let last_order = last_order.first().unwrap();
    let (id, symbol, side, quantity, price, _) = last_order;

    let id = id.to_i32().unwrap();
    let price = price.to_f64().unwrap();
    let quantity = quantity.to_f64().unwrap();
    let symbol = symbol.to_string();
    let side = side.to_string();

    let max_price = fetch_tickers(&symbol, price);    
    let params = (id, symbol, side, quantity, max_price);

    params
}

fn send_message_to_tg(msg: String) -> () {
    let token = env::var("TOKEN").unwrap();
    let chat_id = env::var("CHAT_ID").unwrap();
    let tg_url = format!(
        "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
        token,
        chat_id,
        msg
    );

    let client = Client::new();
    client.post(tg_url).send().expect("Unable to send message to telegram");
}

pub fn manage_limit_order() -> () {
    let conn = establish_connection();

    let params = get_params_for_limit_order();
    let (id, symbol, side, quantity, price) = params;

    let acc: Binance = Binance {
		api_key: env::var("KEY").unwrap(),
		api_secret: env::var("SECRET").unwrap()
	};

    acc.send_limit_order(&symbol, &side, quantity, price);

    let tg_msg: String = format!(
        "Your order was send just now! Symbol: {}, price: {}, side: {}", 
        &symbol, price, &side
    );
    send_message_to_tg(tg_msg);

    let status: bool = acc.is_limit_order_completed();
    if status {
        send_message_to_tg("Your order is filled".to_string());
        update_order(&conn, id);
    }
}
