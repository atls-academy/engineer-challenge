package domain

import (
	"context"
	"time"
)

type UserRepository interface {
	CreateUser(ctx context.Context, user *User) (string, error)
	GetByEmail(ctx context.Context, email string) (*User, error)
	UpdatePassword(ctx context.Context, userID string, newHash string) error
	GetByResetToken(ctx context.Context, token string) (*User, error)
	SetResetToken(ctx context.Context, email string, token string, expires time.Time) error
}
