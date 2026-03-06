package repository

import (
	"context"

	"github.com/atls-academy/engineer-challenge/internal/domain/identity/aggregate"
	"github.com/atls-academy/engineer-challenge/internal/domain/identity/valueobject"
)

// UserRepository defines the persistence contract for User aggregates.
// This is a domain port — implementation lives in infrastructure.
type UserRepository interface {
	Save(ctx context.Context, user *aggregate.User) error
	FindByID(ctx context.Context, id valueobject.UserID) (*aggregate.User, error)
	FindByEmail(ctx context.Context, email valueobject.Email) (*aggregate.User, error)
	ExistsByEmail(ctx context.Context, email valueobject.Email) (bool, error)
	Update(ctx context.Context, user *aggregate.User) error
}
