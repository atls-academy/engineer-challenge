package domain

import (
	"errors"
	"regexp"
	"time"
	"unicode"

	"github.com/google/uuid"
)

var (
	ErrInvalidEmail    = errors.New("invalid email format")
	ErrInvalidPassword = errors.New("password must be at least 8 characters long")
	ErrPasswordTooWeak = errors.New("password must contain at least one uppercase letter, one lowercase letter, one digit, and one special character")
)

type UserID uuid.UUID

func (id UserID) String() string {
	return uuid.UUID(id).String()
}

type Email string

func NewEmail(e string) (Email, error) {
	regex := regexp.MustCompile(`^[a-z0-9._%+\-]+@[a-z0-9.\-]+\.[a-z]{2,4}$`)
	if !regex.MatchString(e) {
		return "", ErrInvalidEmail
	}
	return Email(e), nil
}

type User struct {
	ID           UserID
	Email        Email
	PasswordHash string
	CreatedAt    time.Time
	UpdatedAt    time.Time
}

func NewUser(email Email, passwordHash string) *User {
	return &User{
		ID:           UserID(uuid.New()),
		Email:        email,
		PasswordHash: passwordHash,
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	}
}

func ValidatePasswordComplexity(password string) error {
	if len(password) < 8 {
		return ErrInvalidPassword
	}

	var (
		hasUpper   = false
		hasLower   = false
		hasNumber  = false
		hasSpecial = false
	)

	for _, char := range password {
		switch {
		case unicode.IsUpper(char):
			hasUpper = true
		case unicode.IsLower(char):
			hasLower = true
		case unicode.IsDigit(char):
			hasNumber = true
		case unicode.IsPunct(char) || unicode.IsSymbol(char):
			hasSpecial = true
		}
	}

	if !hasUpper || !hasLower || !hasNumber || !hasSpecial {
		return ErrPasswordTooWeak
	}

	return nil
}

type ResetToken struct {
	Token     string
	UserID    UserID
	ExpiresAt time.Time
}

func (rt *ResetToken) IsExpired() bool {
	return time.Now().After(rt.ExpiresAt)
}
