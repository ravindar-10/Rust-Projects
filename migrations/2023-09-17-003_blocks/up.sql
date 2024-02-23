CREATE TABLE blocks
(
    block_number   SERIAL PRIMARY KEY,
    block_hash VARCHAR(64) UNIQUE NOT NULL,
    transaction_count INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL
);

CREATE INDEX idx_block_created_at ON blocks (created_at);
CREATE INDEX idx_block_hash ON blocks (block_hash);