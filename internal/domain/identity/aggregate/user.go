package aggregate

import (
	"fmt"
	"time"

	"github.com/atls-academy/engineer-challenge/internal/domain/identity/event"
	"github.com/atls-academy/engineer-challenge/internal/domain/identity/valueobject"
)

// UserStatus represents the lifecycle state of a user.
type UserStatus string

const (
	UserStatusActive  UserStatus = "active"
	UserStatusBlocked UserStatus = "blocked"
)

// User is the aggregate root for the Identity bounded context.
type User struct {
	ID           valueobject.UserID
	Email        valueobject.Email
	PasswordHash string
	Status       UserStatus
	ResetToken   *valueobject.ResetToken
	CreatedAt    time.Time
	UpdatedAt    time.Time

	// Domain events collected during the lifecycle of this aggregate.
	events []event.DomainEvent
}

// Register creates a new User aggregate from validated inputs.
// passwordHash must already be computed by the infrastructure layer.
func Register(email valueobject.Email, passwordHash string) (*User, error) {
	if passwordHash == "" {
		return nil, fmt.Errorf("password hash cannot be empty")
	}
	id := valueobject.NewUserID()
	now := time.Now()

	user := &User{
		ID:           id,
		Email:        email,
		PasswordHash: passwordHash,
		Status:       UserStatusActive,
		CreatedAt:    now,
		UpdatedAt:    now,
	}

	user.record(event.NewUserRegistered(id.String(), email.String()))
	return user, nil
}

// Reconstruct hydrates a User aggregate from persistence (no events emitted).
func Reconstruct(id valueobject.UserID, email valueobject.Email, passwordHash string, status UserStatus, resetToken *valueobject.ResetToken, createdAt, updatedAt time.Time) *User {
	return &User{
		ID:           id,
		Email:        email,
		PasswordHash: passwordHash,
		Status:       status,
		ResetToken:   resetToken,
		CreatedAt:    createdAt,
		UpdatedAt:    updatedAt,
	}
}

// RequestPasswordReset generates a new reset token.
// Invariant: user must be active, and no valid token should already exist.
func (u *User) RequestPasswordReset() (*valueobject.ResetToken, error) {
	if u.Status != UserStatusActive {
		return nil, fmt.Errorf("cannot request password reset for non-active user")
	}

	token, err := valueobject.NewResetToken()
	if err != nil {
		return nil, err
	}
	u.ResetToken = &token
	u.UpdatedAt = time.Now()

	u.record(event.NewPasswordResetRequested(u.ID.String(), u.Email.String()))
	return &token, nil
}

// ResetPassword applies a new password using a valid reset token.
func (u *User) ResetPassword(token string, newPasswordHash string) error {
	if u.ResetToken == nil {
		return fmt.Errorf("no reset token issued")
	}
	if !u.ResetToken.IsValid() {
		return fmt.Errorf("reset token is expired or already used")
	}
	if u.ResetToken.Token != token {
		return fmt.Errorf("invalid reset token")
	}
	if newPasswordHash == "" {
		return fmt.Errorf("new password hash cannot be empty")
	}

	used := u.ResetToken.MarkUsed()
	u.ResetToken = &used
	u.PasswordHash = newPasswordHash
	u.UpdatedAt = time.Now()

	u.record(event.NewPasswordResetCompleted(u.ID.String()))
	return nil
}

func (u *User) IsActive() bool {
	return u.Status == UserStatusActive
}

// Events returns and clears collected domain events.
func (u *User) Events() []event.DomainEvent {
	events := u.events
	u.events = nil
	return events
}

func (u *User) record(e event.DomainEvent) {
	u.events = append(u.events, e)
}
