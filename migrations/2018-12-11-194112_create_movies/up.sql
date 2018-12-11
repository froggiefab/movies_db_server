CREATE TABLE movies (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    synopsis TEXT NOT NULL,
    poster VARCHAR,
    rating INTEGER
)