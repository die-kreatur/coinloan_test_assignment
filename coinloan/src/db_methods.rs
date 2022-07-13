use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv;
use std::env;
use crate::db_model::{NewOrder, Order};


pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL is not provided");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_order<'a>(conn: &PgConnection, symbol: &'a str, side: &'a str, quantity: &'a BigDecimal, price: &'a BigDecimal) -> Order {
    use crate::schema::orders;

    let new_order = NewOrder {
        symbol: symbol,
        side: side,
        time_in_force: "GTC",
        quantity: quantity,
        price: price
    };

    diesel::insert_into(orders::table)
        .values(&new_order)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn update_order<'a>(connection: &PgConnection, id: i32) {
    use crate::schema::orders::dsl::{orders, is_comleted};

    diesel::update(orders.find(id))
        .set(is_comleted.eq(true))
        .get_result::<Order>(connection)
        .expect(&format!("Unable to find post {}", id));
}

// TODO: test func
pub fn delete_order<'a>(connection: &PgConnection, id: i32) {
    use crate::schema::orders::dsl::orders;

    diesel::delete(orders.find(id))
        .execute(connection)
        .expect("Error deleting posts");
}

// fn main() {
//     let connection = establish_connection();
//     // let q = BigDecimal::from_f64(0.1).unwrap();
//     // let p = BigDecimal::from_f64(50.0).unwrap();
//     // create_order(&connection, "BTCUSD", "BUY", &q, &p);
//     update_order(&connection, 1);
// }
