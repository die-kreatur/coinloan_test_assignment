pub mod binance_client {

    use hmac::{Hmac, Mac, NewMac};
    use reqwest::{header, Client};
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

        pub async fn send_limit_order(&self, symbol: &str, side: &str, time_in_force: &str, quantity: &str, price: &str) -> Value {
            let ts = Binance::get_timestamp();
            let params = format!(
                "symbol={}&side={}&type=LIMIT&timeInForce={}&quantity={}&price={}&timestamp={}",
                symbol, side, time_in_force, quantity, price, ts
            );
            let signature = self.get_signature(&params);

            let request = format!(
                "https://api.binance.com/api/v3/order/test?{}&signature={}",
                params,
                signature
            );

            let client = self.get_client();
            let response = client.post(request).send().await.unwrap();
            let data: Value = response.json().await.unwrap();

            data
        }
    }
}
