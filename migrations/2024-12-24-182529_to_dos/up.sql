-- Your SQL goes here
CREATE TABLE to_do(
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE
)