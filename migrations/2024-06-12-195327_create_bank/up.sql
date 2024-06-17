-- Your SQL goes here
CREATE TABLE accounts (
  id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  name VARCHAR NOT NULL,
  account VARCHAR NOT NULL,
  UNIQUE (name, account)
)
