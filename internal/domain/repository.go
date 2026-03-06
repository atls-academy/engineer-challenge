package domain

import (
	"context"
)

type UserRepository interface {
	UserReadRepository
	UserWriteRepository
}

type UserReadRepository interface {
	FindByEmail(ctx context.Context, email Email) (*User, error)
	FindByID(ctx context.Context, id UserID) (*User, error)
}

type UserWriteRepository interface {
	Save(ctx context.Context, user *User) error
	Update(ctx context.Context, user *User) error

	SaveResetToken(ctx context.Context, token *ResetToken) error
	FindResetToken(ctx context.Context, token string) (*ResetToken, error)
	DeleteResetToken(ctx context.Context, token string) error
	DeleteResetTokensByUserID(ctx context.Context, userID UserID) error
}
