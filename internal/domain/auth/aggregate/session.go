package aggregate

import (
	"fmt"
	"time"

	"github.com/atls-academy/engineer-challenge/internal/domain/auth/event"
	"github.com/atls-academy/engineer-challenge/internal/domain/auth/valueobject"
)

const (
	AccessTokenTTL  = 15 * time.Minute
	RefreshTokenTTL = 7 * 24 * time.Hour
	MaxSessions     = 5
)

// Session is the aggregate root for the Authentication bounded context.
type Session struct {
	ID           valueobject.SessionID
	UserID       string
	RefreshToken string
	UserAgent    string
	IP           string
	ExpiresAt    time.Time
	CreatedAt    time.Time
	RevokedAt    *time.Time

	events []event.DomainEvent
}

// CreateSession creates a new authentication session.
func CreateSession(userID, refreshToken, userAgent, ip string) (*Session, error) {
	if userID == "" {
		return nil, fmt.Errorf("user ID is required")
	}
	if refreshToken == "" {
		return nil, fmt.Errorf("refresh token is required")
	}

	id := valueobject.NewSessionID()
	now := time.Now()

	session := &Session{
		ID:           id,
		UserID:       userID,
		RefreshToken: refreshToken,
		UserAgent:    userAgent,
		IP:           ip,
		ExpiresAt:    now.Add(RefreshTokenTTL),
		CreatedAt:    now,
	}

	session.record(event.NewUserAuthenticated(id.String(), userID))
	return session, nil
}

func ReconstructSession(id valueobject.SessionID, userID, refreshToken, userAgent, ip string, expiresAt, createdAt time.Time, revokedAt *time.Time) *Session {
	return &Session{
		ID:           id,
		UserID:       userID,
		RefreshToken: refreshToken,
		UserAgent:    userAgent,
		IP:           ip,
		ExpiresAt:    expiresAt,
		CreatedAt:    createdAt,
		RevokedAt:    revokedAt,
	}
}

func (s *Session) IsActive() bool {
	return s.RevokedAt == nil && time.Now().Before(s.ExpiresAt)
}

func (s *Session) Revoke() error {
	if s.RevokedAt != nil {
		return fmt.Errorf("session already revoked")
	}
	now := time.Now()
	s.RevokedAt = &now
	s.record(event.NewSessionRevoked(s.ID.String()))
	return nil
}

func (s *Session) RotateRefreshToken(newToken string) {
	s.RefreshToken = newToken
	s.ExpiresAt = time.Now().Add(RefreshTokenTTL)
}

func (s *Session) Events() []event.DomainEvent {
	events := s.events
	s.events = nil
	return events
}

func (s *Session) record(e event.DomainEvent) {
	s.events = append(s.events, e)
}
