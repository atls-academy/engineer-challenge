-- Index on reset_tokens.user_id for efficient deletion of all tokens per user
-- (used in DeleteResetTokensByUserID called on password reset/change)
CREATE INDEX IF NOT EXISTS idx_reset_tokens_user_id ON reset_tokens (user_id);
