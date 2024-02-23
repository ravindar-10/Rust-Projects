CREATE TABLE transaction_types
(
    transaction_type_id SERIAL PRIMARY KEY,
    type_name           VARCHAR(50) UNIQUE NOT NULL
);

INSERT INTO transaction_types (type_name)
VALUES ('Deposit'),
       ('Withdrawal'),
       ('Transfer'),
       ('Account_Open'),
       ('Account_Update'),
       ('Account_Close');