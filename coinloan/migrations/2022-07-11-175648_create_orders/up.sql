-- Your SQL goes here
CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    symbol VARCHAR(6) NOT NULL,
    side VARCHAR(4) NOT NULL,
    time_in_force VARCHAR(3) DEFAULT 'GTC' NOT NULL,
    quantity DECIMAL NOT NULL,
    price DECIMAL NOT NULL,
    is_comleted BOOLEAN NOT NULL DEFAULT 'f'
);
