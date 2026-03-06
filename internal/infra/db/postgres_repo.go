package db

import (
	"context"
	"errors"

	"github.com/atrump/engineer-challenge/internal/domain"
	"github.com/google/uuid"
	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgconn"
)

type DBPool interface {
	Exec(ctx context.Context, sql string, arguments ...any) (pgconn.CommandTag, error)
	QueryRow(ctx context.Context, sql string, args ...any) pgx.Row
}

type PostgresUserRepository struct {
	pool DBPool
}

func NewPostgresUserRepository(pool DBPool) *PostgresUserRepository {
	return &PostgresUserRepository{pool: pool}
}

func (r *PostgresUserRepository) Save(ctx context.Context, user *domain.User) error {
	query := `INSERT INTO users (id, email, password_hash, created_at, updated_at) 
              VALUES ($1, $2, $3, $4, $5)`
	_, err := r.pool.Exec(ctx, query, user.ID, user.Email, user.PasswordHash, user.CreatedAt, user.UpdatedAt)
	return err
}

func (r *PostgresUserRepository) FindByEmail(ctx context.Context, email domain.Email) (*domain.User, error) {
	query := `SELECT id, email, password_hash, created_at, updated_at FROM users WHERE email = $1`
	var user domain.User
	err := r.pool.QueryRow(ctx, query, string(email)).Scan((*uuid.UUID)(&user.ID), &user.Email, &user.PasswordHash, &user.CreatedAt, &user.UpdatedAt)
	if err != nil {
		if errors.Is(err, pgx.ErrNoRows) {
			return nil, nil
		}
		return nil, err
	}
	return &user, nil
}

func (r *PostgresUserRepository) FindByID(ctx context.Context, id domain.UserID) (*domain.User, error) {
	query := `SELECT id, email, password_hash, created_at, updated_at FROM users WHERE id = $1`
	var user domain.User
	err := r.pool.QueryRow(ctx, query, id).Scan((*uuid.UUID)(&user.ID), &user.Email, &user.PasswordHash, &user.CreatedAt, &user.UpdatedAt)
	if err != nil {
		if errors.Is(err, pgx.ErrNoRows) {
			return nil, nil
		}
		return nil, err
	}
	return &user, nil
}

func (r *PostgresUserRepository) Update(ctx context.Context, user *domain.User) error {
	query := `UPDATE users SET email = $2, password_hash = $3, updated_at = $4 WHERE id = $1`
	_, err := r.pool.Exec(ctx, query, user.ID, user.Email, user.PasswordHash, user.UpdatedAt)
	return err
}

func (r *PostgresUserRepository) SaveResetToken(ctx context.Context, token *domain.ResetToken) error {
	query := `INSERT INTO reset_tokens (token_hash, user_id, expires_at) VALUES ($1, $2, $3)`
	_, err := r.pool.Exec(ctx, query, token.Token, token.UserID, token.ExpiresAt)
	return err
}

func (r *PostgresUserRepository) FindResetToken(ctx context.Context, tokenStr string) (*domain.ResetToken, error) {
	query := `SELECT token_hash, user_id, expires_at FROM reset_tokens WHERE token_hash = $1`
	var token domain.ResetToken
	err := r.pool.QueryRow(ctx, query, tokenStr).Scan(&token.Token, (*uuid.UUID)(&token.UserID), &token.ExpiresAt)
	if err != nil {
		if errors.Is(err, pgx.ErrNoRows) {
			return nil, nil
		}
		return nil, err
	}
	return &token, nil
}

func (r *PostgresUserRepository) DeleteResetToken(ctx context.Context, tokenStr string) error {
	query := `DELETE FROM reset_tokens WHERE token_hash = $1`
	_, err := r.pool.Exec(ctx, query, tokenStr)
	return err
}

func (r *PostgresUserRepository) DeleteResetTokensByUserID(ctx context.Context, userID domain.UserID) error {
	query := `DELETE FROM reset_tokens WHERE user_id = $1`
	_, err := r.pool.Exec(ctx, query, userID)
	return err
}
