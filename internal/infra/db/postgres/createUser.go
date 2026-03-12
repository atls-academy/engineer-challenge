package postgres

import (
	"context"
	"fmt"

	"github.com/Aidajy111/engineer-challenge/internal/domain"
)

func (r *PostgresRepo) CreateUser(ctx context.Context, user *domain.User) (string, error) {
	query := "INSERT INTO users (id, email, password_hash, display_name, status, created_at) VALUES ($1, $2, $3, $4, $5, $6)"

	_, err := r.db.ExecContext(ctx, query, user.ID, user.Email, user.PasswordHash, user.DisplayName, user.Status, user.CreatedAt)
	if err != nil {
		return "", fmt.Errorf("failed to create user: %w", err)
	}

	return user.ID.String(), nil
}
