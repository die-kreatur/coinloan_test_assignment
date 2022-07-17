use jsonrpc_core::Result as JsonRpcResult;
use jsonrpc_derive::rpc;
use jsonrpc_http_server::ServerBuilder;
use bigdecimal::{BigDecimal, FromPrimitive};
use crate::db_methods::{establish_connection, create_order, list_orders, delete_order};
use crate::services::manage_limit_order;

// curl -X GET -H "Content-Type: application/json" --data-binary "{\"jsonrpc\": "\2.0\", "\method\": "\listOrders\", "\id\": 1 }" 127.0.0.1:3030

#[rpc]
pub trait Rpc {

     #[rpc(name = "newOrder")]
     fn add_new_order(&self, symbol: String, side: String, quantity: f64, price: f64) -> JsonRpcResult<()>;

     #[rpc(name = "listOrders")]
     fn list_orders(&self) -> JsonRpcResult<Vec<(i32, String, String, BigDecimal, BigDecimal, bool)>>;

     #[rpc(name = "deleteOrder")]
     fn remove_order(&self, id: i32) -> JsonRpcResult<()>;
}

struct RpcImpl;

impl Rpc for RpcImpl {

     fn add_new_order(&self, symbol: String, side: String, quantity: f64, price: f64) -> JsonRpcResult<()> {
          let conn = establish_connection();
          let quantity = BigDecimal::from_f64(quantity).expect("Not a f64 number");
          let price = BigDecimal::from_f64(price).expect("Not a f64 number");
          let new_order = create_order(&conn, &symbol, &side, &quantity, &price);
          Ok(manage_limit_order(&conn, new_order))
     }

     fn list_orders(&self) -> JsonRpcResult<Vec<(i32, String, String, BigDecimal, BigDecimal, bool)>> {
          let conn = establish_connection();
          Ok(list_orders(&conn))
     }

     fn remove_order(&self, pk: i32) -> JsonRpcResult<()> {
          let conn = establish_connection();
          Ok(delete_order(&conn, pk))
     }
}

pub fn start_jsonrpc_server() {
     let mut io = jsonrpc_core::IoHandler::new();
     io.extend_with(RpcImpl.to_delegate());

     let server = ServerBuilder::new(io)
          .threads(3)
          .start_http(&"127.0.0.1:3030".parse().unwrap())
          .unwrap();

     server.wait();
}
