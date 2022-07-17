mod binance_client;
mod get_prices;
mod db_model;
mod db_methods;
mod schema;
mod json_rpc;
mod services;

#[macro_use]
extern crate diesel;

use crate::json_rpc::start_jsonrpc_server;


fn main() {
	start_jsonrpc_server();
}
