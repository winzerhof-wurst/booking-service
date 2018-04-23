CREATE TABLE rooms (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    bookable BOOL NOT NULL default false
);
