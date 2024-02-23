CREATE TABLE accounts
(
    account_id              SERIAL PRIMARY KEY,
    account_number          VARCHAR(20) UNIQUE             NOT NULL, --IRONCLAD001000000001
    user_id                 INT REFERENCES users (user_id),
    balance                 DOUBLE PRECISION DEFAULT 0.00  NOT NULL,
    account_type_id         INT REFERENCES account_types (account_type_id),
    latest_transaction_hash VARCHAR REFERENCES transactions (transaction_hash),
    created_date            TIMESTAMP        DEFAULT NOW() NOT NULL,
    updated_date            TIMESTAMP        DEFAULT NOW() NOT NULL
);

CREATE INDEX idx_account_user ON accounts (user_id);
CREATE INDEX idx_account_number ON accounts (account_number);

ALTER TABLE accounts
    ADD CONSTRAINT fk_account_user FOREIGN KEY (user_id) REFERENCES users (user_id);
ALTER TABLE accounts
    ADD CONSTRAINT fk_account_type FOREIGN KEY (account_type_id) REFERENCES account_types (account_type_id);
ALTER TABLE accounts
    ADD CONSTRAINT fk_latest_transaction FOREIGN KEY (latest_transaction_hash) REFERENCES transactions (transaction_hash);
ALTER TABLE accounts
    ADD COLUMN is_deleted BOOLEAN DEFAULT FALSE NOT NULL;