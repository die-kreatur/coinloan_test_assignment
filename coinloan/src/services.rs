use crate::db_methods::update_order;
use bigdecimal::ToPrimitive;
use crate::get_prices::fetch_tickers;
use std::env;
use reqwest::blocking::Client;
use crate::binance_client::Binance;
use crate::db_model::Order;
use diesel::pg::PgConnection;


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

pub fn manage_limit_order(conn: &PgConnection, new_order: Order) -> () {

    let price = new_order.price.to_f64().unwrap();
    let quantity = new_order.quantity.to_f64().unwrap();

    let max_price = fetch_tickers(&new_order.symbol, price);

    let acc: Binance = Binance {
		api_key: env::var("KEY").unwrap(),
		api_secret: env::var("SECRET").unwrap()
	};

    acc.send_limit_order(&new_order.symbol, &new_order.side, quantity, max_price);

    let tg_msg: String = format!(
        "Your order was sent just now! Symbol: {}, price: {}, side: {}", 
        &new_order.symbol, price, &new_order.side
    );
    send_message_to_tg(tg_msg);

    let status: bool = acc.is_limit_order_completed();
    if status {
        send_message_to_tg("Your order is filled".to_string());
        update_order(&conn, new_order.id);
    }
}
