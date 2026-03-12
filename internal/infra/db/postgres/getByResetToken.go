package postgres

import (
	"context"

	"github.com/Aidajy111/engineer-challenge/internal/domain"
)

func (r *PostgresRepo) GetByResetToken(ctx context.Context, token string) (*domain.User, error) {
	var user domain.User
	query := `SELECT id, email, password_hash, reset_token_expires FROM users WHERE reset_token = $1`

	err := r.db.QueryRowxContext(ctx, query, token).Scan(
		&user.ID, &user.Email, &user.PasswordHash, &user.ResetTokenExpires,
	)
	if err != nil {
		return nil, err
	}
	return &user, nil
}
