use super::schema::orders;
use bigdecimal::BigDecimal;


#[derive(Queryable)]
pub struct Order {
    pub id: i32,
    pub symbol: String,
    pub side: String,
    pub time_in_force: String,
    pub quantity: BigDecimal,
    pub price: BigDecimal,
    pub is_completed: bool
}

#[derive(Insertable)]
#[table_name="orders"]
pub struct NewOrder<'a> {
    pub symbol: &'a str,
    pub side: &'a str,
    pub time_in_force: &'a str,
    pub quantity: &'a BigDecimal,
    pub price: &'a BigDecimal,
}
