CREATE SCHEMA IF NOT EXISTS rs_market;
CREATE TYPE gift_card_status AS ENUM ('Available', 'Sold', 'Redeemed');

CREATE TABLE IF NOT EXISTS rs_market.gift_cards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    amount DECIMAL(12,2) NOT NULL,
    status gift_card_status NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    version INTEGER NOT NULL DEFAULT 0
);