use jsonrpc_core_client::transports::local;
use jsonrpc_core::futures::{self, future};
use jsonrpc_core::{IoHandler, Result, BoxFuture};
use jsonrpc_derive::rpc;
use jsonrpc_http_server::ServerBuilder;
use bigdecimal::{BigDecimal, FromPrimitive};
use crate::db_methods::{establish_connection, create_order, list_orders, delete_order};

// TODO: test this module

#[rpc]
pub trait Rpc {
    #[rpc(name = "newOrder")]
    fn add_new_order(&self, symbol: &str, side: &str, quantity: f64, price: f64) -> ();

    #[rpc(name = "listOrders")]
    fn list_orders(&self) -> Vec<(String, String, BigDecimal, BigDecimal, bool)>;

    #[rpc(name = "deleteOrder")]
    fn remove_order(&self) -> ();
}

struct RpcImpl;

impl Rpc for RpcImpl {
   fn add_new_order(&self, symbol: &str, side: &str, quantity: f64, price: f64) -> () {
        let conn = establish_connection();
        let quantity = BigDecimal::from_f64(quantity).expect("Not a f64 number");
        let price = BigDecimal::from_f64(price).expect("Not a f64 number");
        create_order(&conn, symbol, side, &quantity, &price);
   }

   fn list_orders(&self) -> Vec<(String, String, BigDecimal, BigDecimal, bool)> {
        let conn = establish_connection();
        list_orders(&conn)
   }

   fn remove_order(&self, pk: i32) -> () {
        let conn = establish_connection();
        delete_order(&conn, pk);
   }
}

fn start_jsonrpc_server() {
	let mut io = jsonrpc_core::IoHandler::new();
	io.extend_with(RpcImpl.to_delegate());

    let server = ServerBuilder::new(io)
		.threads(3)
		.start_http(&"127.0.0.1:3030".parse().unwrap())
		.unwrap();

	server.wait();
}
