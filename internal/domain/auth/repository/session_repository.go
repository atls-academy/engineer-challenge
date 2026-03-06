package repository

import (
	"context"

	"github.com/atls-academy/engineer-challenge/internal/domain/auth/aggregate"
	"github.com/atls-academy/engineer-challenge/internal/domain/auth/valueobject"
)

// SessionRepository defines the persistence contract for Session aggregates.
type SessionRepository interface {
	Save(ctx context.Context, session *aggregate.Session) error
	FindByID(ctx context.Context, id valueobject.SessionID) (*aggregate.Session, error)
	FindByRefreshToken(ctx context.Context, token string) (*aggregate.Session, error)
	FindActiveByUserID(ctx context.Context, userID string) ([]*aggregate.Session, error)
	Update(ctx context.Context, session *aggregate.Session) error
	RevokeAllForUser(ctx context.Context, userID string) error
}
