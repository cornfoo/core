-- +goose Up
-- +goose StatementBegin
CREATE TABLE devices (
    id SERIAL PRIMARY KEY,
    device_id VARCHAR(32) NOT NULL UNIQUE,
    platform_store VARCHAR(32),
    token VARCHAR(256) NOT NULL,
    locale VARCHAR(8) NOT NULL,
    currency VARCHAR(8) NOT NULL,
    is_push_enabled BOOLEAN NOT NULL,
    is_price_alerts_enabled BOOLEAN NOT NULL DEFAULT false,
    version VARCHAR(16) NOT NULL,
    subscriptions_version INTEGER NOT NULL DEFAULT 0,
    os VARCHAR(64),
    model VARCHAR(128),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX devices_token_idx ON devices (token);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_devices_updated_at
    BEFORE UPDATE ON devices
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE devices;
-- +goose StatementEnd
