package postgres

import (
	"context"
	"fmt"

	"github.com/Aidajy111/engineer-challenge/internal/domain"
)

func (r *PostgresRepo) UpdatePassword(ctx context.Context, userID string, newHash string) error {
	query := `UPDATE users SET password_hash = $1 WHERE id = $2`

	result, err := r.db.ExecContext(ctx, query, newHash, userID)
	if err != nil {
		return fmt.Errorf("failed to update password: %w", err)
	}

	// Проверяем, обновилась ли хоть одна строка (был ли такой ID)
	rows, err := result.RowsAffected()
	if err != nil {
		return err
	}
	if rows == 0 {
		return domain.ErrUserNotFound
	}

	return nil
}
