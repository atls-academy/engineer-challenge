package db

import (
	"context"
	"testing"
	"time"

	"github.com/atrump/engineer-challenge/internal/domain"
	"github.com/google/uuid"
	"github.com/pashagolub/pgxmock/v3"
	"github.com/stretchr/testify/assert"
)

func TestPostgresUserRepository_Save(t *testing.T) {
	mock, err := pgxmock.NewPool()
	if err != nil {
		t.Fatal(err)
	}
	defer mock.Close()

	repo := NewPostgresUserRepository(mock)
	ctx := context.Background()

	id := domain.UserID(uuid.New())
	email, _ := domain.NewEmail("test@example.com")
	user := &domain.User{
		ID:           id,
		Email:        email,
		PasswordHash: "hash",
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	}

	mock.ExpectExec("INSERT INTO users").
		WithArgs(user.ID, user.Email, user.PasswordHash, user.CreatedAt, user.UpdatedAt).
		WillReturnResult(pgxmock.NewResult("INSERT", 1))

	err = repo.Save(ctx, user)
	assert.NoError(t, err)

	if err := mock.ExpectationsWereMet(); err != nil {
		t.Errorf("there were unfulfilled expectations: %s", err)
	}
}

func TestPostgresUserRepository_FindByEmail(t *testing.T) {
	mock, err := pgxmock.NewPool()
	if err != nil {
		t.Fatal(err)
	}
	defer mock.Close()

	repo := NewPostgresUserRepository(mock)
	ctx := context.Background()
	email, _ := domain.NewEmail("test@example.com")

	id := uuid.New()
	rows := pgxmock.NewRows([]string{"id", "email", "password_hash", "created_at", "updated_at"}).
		AddRow(id, string(email), "hash", time.Now(), time.Now())

	mock.ExpectQuery("SELECT (.+) FROM users WHERE email =").
		WithArgs(string(email)).
		WillReturnRows(rows)

	user, err := repo.FindByEmail(ctx, email)
	assert.NoError(t, err)
	assert.NotNil(t, user)
	assert.Equal(t, domain.UserID(id), user.ID)

	if err := mock.ExpectationsWereMet(); err != nil {
		t.Errorf("there were unfulfilled expectations: %s", err)
	}
}

func TestPostgresUserRepository_Update(t *testing.T) {
	mock, err := pgxmock.NewPool()
	if err != nil {
		t.Fatal(err)
	}
	defer mock.Close()

	repo := NewPostgresUserRepository(mock)
	ctx := context.Background()

	id := domain.UserID(uuid.New())
	email, _ := domain.NewEmail("updated@example.com")
	user := &domain.User{
		ID:           id,
		Email:        email,
		PasswordHash: "new-hash",
		UpdatedAt:    time.Now(),
	}

	mock.ExpectExec("UPDATE users SET email =").
		WithArgs(user.ID, user.Email, user.PasswordHash, user.UpdatedAt).
		WillReturnResult(pgxmock.NewResult("UPDATE", 1))

	err = repo.Update(ctx, user)
	assert.NoError(t, err)

	if err := mock.ExpectationsWereMet(); err != nil {
		t.Errorf("there were unfulfilled expectations: %s", err)
	}
}

func TestPostgresUserRepository_ResetTokens(t *testing.T) {
	mock, err := pgxmock.NewPool()
	if err != nil {
		t.Fatal(err)
	}
	defer mock.Close()

	repo := NewPostgresUserRepository(mock)
	ctx := context.Background()
	userID := domain.UserID(uuid.New())
	tokenStr := "reset-token"
	expiresAt := time.Now().Truncate(time.Second)

	// Test SaveResetToken
	mock.ExpectExec("INSERT INTO reset_tokens").
		WithArgs(tokenStr, userID, expiresAt).
		WillReturnResult(pgxmock.NewResult("INSERT", 1))

	err = repo.SaveResetToken(ctx, &domain.ResetToken{
		Token:     tokenStr,
		UserID:    userID,
		ExpiresAt: expiresAt,
	})
	assert.NoError(t, err)

	// Test FindResetToken
	rows := pgxmock.NewRows([]string{"token_hash", "user_id", "expires_at"}).
		AddRow(tokenStr, uuid.UUID(userID), expiresAt)

	mock.ExpectQuery("SELECT (.+) FROM reset_tokens WHERE token_hash =").
		WithArgs(tokenStr).
		WillReturnRows(rows)

	token, err := repo.FindResetToken(ctx, tokenStr)
	assert.NoError(t, err)
	assert.NotNil(t, token)
	assert.Equal(t, userID, token.UserID)

	// Test DeleteResetToken
	mock.ExpectExec("DELETE FROM reset_tokens WHERE token_hash =").
		WithArgs(tokenStr).
		WillReturnResult(pgxmock.NewResult("DELETE", 1))

	err = repo.DeleteResetToken(ctx, tokenStr)
	assert.NoError(t, err)

	if err := mock.ExpectationsWereMet(); err != nil {
		t.Errorf("there were unfulfilled expectations: %s", err)
	}
}
