extern crate reqwest;
extern crate serde_json;
extern crate time;
extern crate crypto;
extern crate rustc_serialize;
extern crate url;

use serde_json::{Value, Error};
use std::collections::HashMap;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;
use rustc_serialize::hex::ToHex;
use reqwest::header::Headers;

struct Nonce {
    nonce: i64,
}

impl Nonce {
    pub fn new() -> Nonce {
        let sec = time::now().to_timespec().sec;
        let nsec = time::now().to_timespec().nsec;
        Nonce { nonce: sec as i64 * 1_000_000_000 + nsec as i64 }
    }
}

impl Iterator for Nonce {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        self.nonce += 1;
        Some(self.nonce)
    }
}

pub struct Client {
    key: &'static str,
    secret: &'static str,
    public_base: &'static str,
    private_base: &'static str,
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(key: &'static str, secret: &'static str) -> Client {
        Client {
            key: key,
            secret: secret,
            public_base: "https://public.bitbank.cc/",
            private_base: "https://api.bitbank.cc",
            http_client: reqwest::Client::new().unwrap(),
        }
    }

    pub fn get_ticker(&self, pair: &str) -> Result<HashMap<String, Value>, Error> {
        let url = &format!("{}{}/ticker", self.public_base, pair);
        let resp = self.http_client.get(url).send().unwrap();
        let value: HashMap<String, Value> = serde_json::from_reader(resp).unwrap();
        Ok(value)
    }

    pub fn get_depth(&self, pair: &str) -> Result<HashMap<String, Value>, Error> {
        let url = &format!("{}{}/depth", self.public_base, pair);
        let resp = self.http_client.get(url).send().unwrap();
        let value: HashMap<String, Value> = serde_json::from_reader(resp).unwrap();
        Ok(value)
    }

    pub fn get_transactions(&self, pair: &str) -> Result<HashMap<String, Value>, Error> {
        let url = &format!("{}{}/transactions", self.public_base, pair);
        let resp = self.http_client.get(url).send().unwrap();
        let value: HashMap<String, Value> = serde_json::from_reader(resp).unwrap();
        Ok(value)
    }

    pub fn get_transactions_by_date(&self,
                                    pair: &str,
                                    date: &str)
                                    -> Result<HashMap<String, Value>, Error> {
        let url = &format!("{}{}/transactions/{}", self.public_base, pair, date);
        let resp = self.http_client.get(url).send().unwrap();
        let value: HashMap<String, Value> = serde_json::from_reader(resp).unwrap();
        Ok(value)
    }

    pub fn get_candlestick(&self,
                           pair: &str,
                           candle_type: &str,
                           date: &str)
                           -> Result<HashMap<String, Value>, Error> {
        let url = &format!("{}{}/candlestick/{}/{}",
                self.public_base,
                pair,
                candle_type,
                date);
        let resp = self.http_client.get(url).send().unwrap();
        let value: HashMap<String, Value> = serde_json::from_reader(resp).unwrap();
        Ok(value)
    }

    pub fn get_assets(&self) -> Result<HashMap<String, Value>, Error> {
        let path = "/v1/user/assets";
        let resp = self.private_get_request(path, "");
        let value: HashMap<String, Value> = serde_json::from_reader(resp).unwrap();
        Ok(value)
    }

    pub fn get_order(&self, pair: &str, order_id: &str) -> Result<HashMap<String, Value>, Error> {
        let path = "/v1/user/spot/order";
        let query = &format!("?pair={}&order_id={}", pair, order_id);
        let resp = self.private_get_request(path, query);
        let value: HashMap<String, Value> = serde_json::from_reader(resp).unwrap();
        Ok(value)
    }

    pub fn order(&self,
                 pair: &str,
                 amount: &str,
                 price: &str,
                 order_side: &str,
                 order_type: &str)
                 -> Result<HashMap<String, Value>, Error> {
        let path = "/v1/user/spot/order";
        let mut body = HashMap::new();
        body.insert("pair", pair);
        body.insert("amount", amount);
        body.insert("price", price);
        body.insert("side", order_side);
        body.insert("type", order_type);
        let body = &format!("{:?}", body);

        let resp = self.private_post_request(body, path);
        let value: HashMap<String, Value> = serde_json::from_reader(resp).unwrap();
        Ok(value)
    }

    pub fn cancel_order(&self,
                        pair: &str,
                        order_id: &str)
                        -> Result<HashMap<String, Value>, Error> {
        let path = "/v1/user/spot/cancel_order";
        let mut body = HashMap::new();
        body.insert("pair", pair);
        body.insert("order_id", order_id);
        let body = &format!("{:?}", body);

        let resp = self.private_post_request(body, path);
        let value: HashMap<String, Value> = serde_json::from_reader(resp).unwrap();
        Ok(value)
    }
    /*
    pub fn cancel_orders(&self,
                         pair: &str,
                         order_ids: Vec<i32>)
                         -> Result<HashMap<String, Value>, Error> {
        let path = "/v1/user/spot/cancel_order";
        let mut body = HashMap::new();
        body.insert("pair", pair);
        //body.insert("order_ids", order_ids);
        let body = &format!("{:?}", body);

        let resp = self.private_post_request(body, path);
        let value: HashMap<String, Value> = serde_json::from_reader(resp).unwrap();
        Ok(value)
    }

    pub fn get_orders_info(&self,
                           pair: &str,
                           order_ids: Vec<i32>)
                           -> Result<HashMap<String, Value>, Error> {
        let path = "/v1/user/spot/orders_info";
        let mut body = HashMap::new();
        body.insert("pair", pair);
        //body.insert("order_ids", order_ids);
        let body = &format!("{:?}", body);

        let resp = self.private_post_request(body, path);
        let value: HashMap<String, Value> = serde_json::from_reader(resp).unwrap();
        Ok(value)
    }
	*/
    pub fn get_active_orders(&self,
                             pair: &str,
                             count: &str,
                             from_id: &str,
                             end_id: &str,
                             since: &str,
                             end: &str)
                             -> Result<HashMap<String, Value>, Error> {
        let path = "/v1/user/spot/active_orders";
        let query = &format!("?pair={}&count={}&from_id={}&end_id={}&since={}&end={}",
                pair,
                count,
                from_id,
                end_id,
                since,
                end);
        let resp = self.private_get_request(path, query);
        let value: HashMap<String, Value> = serde_json::from_reader(resp).unwrap();
        Ok(value)
    }

    pub fn get_withdrawal_account(&self, asset: &str) -> Result<HashMap<String, Value>, Error> {
        let path = "/v1/user/withdrawal_account";
        let query = &format!("?asset={}", asset);
        let resp = self.private_get_request(path, query);
        let value: HashMap<String, Value> = serde_json::from_reader(resp).unwrap();
        Ok(value)
    }

    pub fn request_withdrawal(&self, asset: &str) -> Result<HashMap<String, Value>, Error> {
        let path = "/v1/user/request_withdrawal";
        let mut body = HashMap::new();
        body.insert("asset", asset);
        let body = &format!("{:?}", body);

        let resp = self.private_post_request(body, path);
        let value: HashMap<String, Value> = serde_json::from_reader(resp).unwrap();
        Ok(value)
    }

    fn private_post_request(&self, body: &str, path: &str) -> reqwest::Response {
        let n = Nonce::new().next().unwrap();
        let message = format!("{}{}", n, body);

        let header = self.create_header(&n.to_string(), &message);

        let url = &format!("{}{}", self.private_base, path);
        let resp = self.http_client.post(url).body(body).headers(header);

        resp.send().unwrap()
    }

    fn private_get_request(&self, path: &str, query: &str) -> reqwest::Response {
        let n = Nonce::new().next().unwrap();
        let message = format!("{}{}{}", n, path, query);
        let header = self.create_header(&n.to_string(), &message);

        let url = &format!("{}{}", self.private_base, path);
        let resp = self.http_client.get(url).headers(header);

        resp.send().unwrap()
    }

    fn create_header(&self, nonce: &str, message: &str) -> Headers {
        let mut header = Headers::new();
        header.set_raw("ACCESS-KEY", vec![self.key.as_bytes().to_vec()]);
        header.set_raw("ACCESS-NONCE", vec![nonce.as_bytes().to_vec()]);
        header.set_raw("ACCESS-SIGNATURE",
                       vec![self.signature(&message).to_string().as_bytes().to_vec()]);

        header
    }

    fn signature(&self, message: &str) -> String {
        let mut mac = Hmac::new(Sha256::new(), self.secret.as_bytes());
        mac.input(message.as_bytes());
        let signature = mac.result().code().to_hex().to_string();

        signature
    }
}


#[cfg(test)]
mod tests {}
