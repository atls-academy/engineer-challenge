package valueobject

import (
	"fmt"
	"net/mail"
	"strings"
)

// Email represents a validated email address value object.
type Email struct {
	address string
}

func NewEmail(address string) (Email, error) {
	normalized := strings.ToLower(strings.TrimSpace(address))
	if normalized == "" {
		return Email{}, fmt.Errorf("email address cannot be empty")
	}
	if _, err := mail.ParseAddress(normalized); err != nil {
		return Email{}, fmt.Errorf("invalid email format: %s", address)
	}
	if len(normalized) > 254 {
		return Email{}, fmt.Errorf("email address too long")
	}
	return Email{address: normalized}, nil
}

func (e Email) String() string {
	return e.address
}

func (e Email) Equals(other Email) bool {
	return e.address == other.address
}
