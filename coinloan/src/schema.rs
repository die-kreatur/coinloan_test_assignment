table! {
    orders (id) {
        id -> Int4,
        symbol -> Varchar,
        side -> Varchar,
        time_in_force -> Varchar,
        quantity -> Numeric,
        price -> Numeric,
        is_comleted -> Bool,
    }
}
