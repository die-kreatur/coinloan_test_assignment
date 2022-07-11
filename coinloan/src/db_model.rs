#[derive(Queryable)]
pub struct Order {
    pub id: i32,
    pub symbol: String,
    pub side: String,
    pub time_in_force: String,
    pub quantity: f64,
    pub price: f64
}
