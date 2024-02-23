CREATE TABLE account_types
(
    account_type_id SERIAL PRIMARY KEY,
    type_name       VARCHAR(50) UNIQUE NOT NULL
);

INSERT INTO account_types (type_name)
VALUES ('Savings'),
       ('Current')