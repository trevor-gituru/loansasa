-- Your SQL goes here
CREATE TABLE IF NOT EXISTS wallets (
    id SERIAL PRIMARY KEY,              -- Unique identifier for the wallet
    starknet_address VARCHAR(255) NOT NULL,    -- StarkNet address of the wallet
    public_key VARCHAR(255) NOT NULL,                   -- Public key of the wallet
    private_key VARCHAR(255) NOT NULL,         -- Encrypted private key of the wallet
    user_id INT REFERENCES users(id)     -- Foreign key referencing users table, NULL means the wallet is available
);