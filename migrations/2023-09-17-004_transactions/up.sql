CREATE TABLE transactions
(
    transaction_id      SERIAL PRIMARY KEY,
    user_id             INT REFERENCES users (user_id),
    transaction_type_id INT REFERENCES transaction_types (transaction_type_id),
    transaction_data    JSONB              NOT NULL,
    transaction_hash    VARCHAR(64) UNIQUE NOT NULL,
    transaction_date    TIMESTAMP                   DEFAULT NOW() NOT NULL,
    block_number        INT REFERENCES blocks (block_number),
    status              VARCHAR(42)        NOT NULL DEFAULT 'PENDING'
);

CREATE INDEX idx_transaction_date ON transactions (transaction_date);
CREATE INDEX idx_transaction_hash ON transactions (transaction_hash);

ALTER TABLE transactions
    ADD CONSTRAINT fk_transaction_user FOREIGN KEY (user_id) REFERENCES users (user_id);
ALTER TABLE transactions
    ADD CONSTRAINT fk_transaction_type FOREIGN KEY (transaction_type_id) REFERENCES transaction_types (transaction_type_id);
ALTER TABLE transactions
    ADD CONSTRAINT fk_transaction_block FOREIGN KEY (block_number) REFERENCES blocks (block_number);
