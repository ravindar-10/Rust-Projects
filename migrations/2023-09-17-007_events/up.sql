CREATE TABLE events
(
    event_id       SERIAL PRIMARY KEY,
    transaction_id INT REFERENCES transactions (transaction_id),
    event_type     VARCHAR(20) NOT NULL,
    event_data     JSON,
    created_at     TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_event_transaction ON events (transaction_id);
CREATE INDEX idx_event_type ON events (event_type);

ALTER TABLE events
    ADD CONSTRAINT fk_event_transaction FOREIGN KEY (transaction_id) REFERENCES transactions (transaction_id);