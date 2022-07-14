use serde_json;
use serde::Deserialize;
use tungstenite::connect;
use tungstenite::Message::Text as WsResponse;


const WS_STREAM: &str = "wss://stream.binance.com:9443/stream?streams=";

#[derive(Deserialize, Debug)]
pub struct ParsedData {
    pub data: Ticker
}

#[derive(Deserialize, Debug)]
pub struct Ticker {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub price: String
}

pub fn fetch_tickers(ticker: &str) -> ParsedData {
    let mut ws_stream = format!("wss://stream.binance.com:9443/stream?streams={}miniTicker", ticker);

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
        
        return parsed_data
    }
}
