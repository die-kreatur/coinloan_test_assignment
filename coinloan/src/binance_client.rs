use hmac::{Hmac, Mac, NewMac};
use reqwest::{header, blocking::Client as Client};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::Value;


pub struct Binance {
    pub api_key: String,
    pub api_secret: String
}

impl Binance {
    
    fn get_signature(&self, params: &String) -> String {
        let secret_key = &self.api_secret;
        let mut signed_key = Hmac::<Sha256>
            ::new_from_slice(secret_key.as_bytes())
            .unwrap();
        signed_key.update(params.as_bytes());
        let signature = hex::encode(signed_key.finalize().into_bytes());

        signature
    }

    fn get_timestamp() -> String {
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        ts.as_millis().to_string()
    }

    pub fn get_client(&self) -> Client {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::HeaderName::from_static("x-mbx-apikey"),
            header::HeaderValue::from_str(&self.api_key).unwrap(),
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        client
    }

    pub fn send_limit_order(&self, symbol: &str, side: &str, quantity: f64, price: f64) -> Value {
        let ts = Binance::get_timestamp();
        let params = format!(
            "symbol={}&side={}&type=LIMIT&timeInForce=GTC&quantity={}&price={}&timestamp={}",
            symbol, side, quantity, price, ts
        );
        let signature = self.get_signature(&params);

        let request = format!(
            "https://api.binance.com/api/v3/order/test?{}&signature={}",
            params,
            signature
        );

        let client = self.get_client();
        let response = client.post(request).send().unwrap();
        let data: Value = response.json().unwrap();

        data
    }

    pub fn get_listen_key(&self) -> String {
        let client = self.get_client();
        let response = client
            .post("https://api.binance.com/api/v1/userDataStream")
            .send()
            .unwrap();

        let listen_key: Value = response.json().unwrap();

        listen_key["listenKey"].to_string()
    }
}
