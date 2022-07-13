#[derive(Queryable)]
pub struct Order {
    pub id: i32,
    pub symbol: String,
    pub side: String,
    pub time_in_force: String,
    pub quantity: f64,
    pub price: f64
}

use super::schema::orders;
use bigdecimal::BigDecimal;

#[derive(Insertable)]
#[table_name="orders"]
pub struct NewOrder<'a> {
    pub symbol: &'a str,
    pub side: &'a str,
    pub time_in_force: &'a str,
    pub quantity: &'a BigDecimal,
    pub price: &'a BigDecimal
}