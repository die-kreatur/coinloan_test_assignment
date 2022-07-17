use serde_json;
use serde::de;
use serde::{Deserialize, Deserializer};
use tungstenite::connect;
use tungstenite::Message::Text as WsResponse;


#[derive(Deserialize)]
struct ParsedData {
    data: Ticker
}

#[derive(Deserialize)]
struct Ticker {
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "c")]
    #[serde(deserialize_with = "string_to_f64")]
    price: f64
}

fn string_to_f64<'a, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'a>,
{
    let str_num = String::deserialize(deserializer).unwrap();
    str_num.parse::<f64>().map_err(de::Error::custom)
}

pub fn fetch_tickers(ticker: &str, to_compare: f64) -> f64 {
    let ws_stream = format!("wss://stream.binance.com:9443/stream?streams={}miniTicker", ticker);

    let (mut ws, _) = 
        connect(ws_stream).expect("Cannot establish connection");

    loop {
        let msg = ws.read_message().expect("Cannot read message");
        let msg = match msg {
            WsResponse(m) => m,
            _ => {
                    panic!("Response is invalid");
                }
            };
        
        let parsed_data: ParsedData = 
            serde_json::from_str(&msg).expect("Unable to parse message");

        if parsed_data.data.price >= to_compare {
            return parsed_data.data.price
        }
    }
}
