use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv;
use std::env;
use crate::db_model::{NewOrder, Order};


/// установление соединения с базой данных
pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL is not provided");
    PgConnection::establish(&database_url)
        .expect("Cannot connect to DB")
}

/// сохранение параметров для ордера в базу данных
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
        .expect("Cannot save an order")
}

/// обновление статуса ордера на полностью исполненный
pub fn update_order<'a>(connection: &PgConnection, id: i32) {
    use crate::schema::orders::dsl::{orders, is_completed};

    diesel::update(orders.find(id))
        .set(is_completed.eq(true))
        .get_result::<Order>(connection)
        .expect("Unable to find requested order");
}

/// удаление ордера из базы данных (не отменяет исполнение на бирже)
pub fn delete_order<'a>(connection: &PgConnection, id: i32) {
    use crate::schema::orders::dsl::orders;

    diesel::delete(orders.find(id))
        .execute(connection)
        .expect("Cannot delete order");
}

/// вывод последних десяти добавленных ордеров
pub fn list_orders<'a>(connection: &PgConnection) -> Vec<(i32, String, String, BigDecimal, BigDecimal, bool)> {
    use crate::schema::orders::dsl::{orders, id, symbol, side, quantity, price, is_completed};

    let results = orders.select((id, symbol, side, quantity, price, is_completed))
        .order_by(id.desc())
        .limit(10)
        .load::<(i32, String, String, BigDecimal, BigDecimal, bool)>(connection)
        .expect("Cannot get orders from DB");

    results
}
