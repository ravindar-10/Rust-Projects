CREATE TABLE users
(
    user_id           SERIAL PRIMARY KEY,
    email             VARCHAR(255) UNIQUE NOT NULL,
    first_name        VARCHAR(100)        NOT NULL,
    last_name         VARCHAR(100)        NOT NULL,
    nonce             INT                 NOT NULL DEFAULT 0,
    registration_date TIMESTAMP           NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_user_email ON users (email);
CREATE INDEX idx_user_first_name ON users (first_name);
CREATE INDEX idx_user_last_name ON users (last_name);

INSERT INTO users (email, first_name, last_name)
VALUES ('root@ironclad.com', 'Root', 'User');