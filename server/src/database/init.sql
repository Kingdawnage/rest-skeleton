CREATE DATABASE rust;
\c rust;

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE Users (
    id UUID NOT NULL DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    age INT
);

INSERT INTO Users (name, password, age) VALUES
    ('Alice', '123456', 20),
    ('Bob', '123456', 21),
    ('Charlie', '123456', 22);