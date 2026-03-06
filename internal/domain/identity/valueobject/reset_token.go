package valueobject

import (
	"crypto/rand"
	"encoding/hex"
	"fmt"
	"time"
)

// ResetToken represents a one-time use password reset token.
type ResetToken struct {
	Token     string
	ExpiresAt time.Time
	Used      bool
}

const (
	ResetTokenTTL    = 15 * time.Minute
	ResetTokenLength = 32 // bytes
)

func NewResetToken() (ResetToken, error) {
	b := make([]byte, ResetTokenLength)
	if _, err := rand.Read(b); err != nil {
		return ResetToken{}, fmt.Errorf("failed to generate reset token: %w", err)
	}
	return ResetToken{
		Token:     hex.EncodeToString(b),
		ExpiresAt: time.Now().Add(ResetTokenTTL),
		Used:      false,
	}, nil
}

func (t ResetToken) IsExpired() bool {
	return time.Now().After(t.ExpiresAt)
}

func (t ResetToken) IsValid() bool {
	return !t.Used && !t.IsExpired()
}

func (t ResetToken) MarkUsed() ResetToken {
	t.Used = true
	return t
}
