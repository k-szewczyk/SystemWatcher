-- Your SQL goes here
CREATE TABLE nodes (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    address VARCHAR NOT NULL UNIQUE
);