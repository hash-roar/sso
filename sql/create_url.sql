DROP TABLE if EXISTS urls;
CREATE TABLE urls(
    id serial PRIMARY KEY,
    short_url varchar(64),
    dest_url varchar(256),
    time TIMESTAMPTZ
);