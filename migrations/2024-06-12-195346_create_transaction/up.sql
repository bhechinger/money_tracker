-- Your SQL goes here
CREATE DOMAIN transaction_type as VARCHAR
  CONSTRAINT check_type CHECK (VALUE IN ('Credit', 'Debit'));

CREATE TABLE transactions (
  id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  account INTEGER NOT NULL REFERENCES accounts(id),
  record_date DATE NOT NULL,
  value_date DATE NOT NULL,
  description VARCHAR NOT NULL,
  t_type transaction_type NOT NULL,
  amount NUMERIC(100,2) NOT NULL,
  balance NUMERIC(100,2) NOT NULL,
  UNIQUE(account, record_date, value_date, description, t_type, amount, balance)
)
