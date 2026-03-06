package domain

import (
	"context"
	"time"
)

// Session represents a user's active login session.
type Session struct {
	Token     string
	UserID    UserID
	ExpiresAt time.Time
}

// SessionRepository defines the interface for storing and retrieving sessions.
type SessionRepository interface {
	Save(ctx context.Context, session *Session) error
	FindByToken(ctx context.Context, token string) (*Session, error)
	Delete(ctx context.Context, token string) error
}
