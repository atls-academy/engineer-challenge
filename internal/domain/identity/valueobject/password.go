package valueobject

import (
	"fmt"
	"unicode"
)

// PasswordPolicy defines password complexity requirements.
type PasswordPolicy struct {
	MinLength      int
	RequireUpper   bool
	RequireLower   bool
	RequireDigit   bool
	RequireSpecial bool
}

var DefaultPasswordPolicy = PasswordPolicy{
	MinLength:      8,
	RequireUpper:   true,
	RequireLower:   true,
	RequireDigit:   true,
	RequireSpecial: true,
}

// Password represents a plaintext password that has been validated against the policy.
// This value object is transient — never persisted. Only PasswordHash is stored.
type Password struct {
	plaintext string
}

func NewPassword(plaintext string, policy PasswordPolicy) (Password, error) {
	if len(plaintext) < policy.MinLength {
		return Password{}, fmt.Errorf("password must be at least %d characters", policy.MinLength)
	}
	if len(plaintext) > 128 {
		return Password{}, fmt.Errorf("password must not exceed 128 characters")
	}

	var hasUpper, hasLower, hasDigit, hasSpecial bool
	for _, ch := range plaintext {
		switch {
		case unicode.IsUpper(ch):
			hasUpper = true
		case unicode.IsLower(ch):
			hasLower = true
		case unicode.IsDigit(ch):
			hasDigit = true
		case unicode.IsPunct(ch) || unicode.IsSymbol(ch):
			hasSpecial = true
		}
	}

	if policy.RequireUpper && !hasUpper {
		return Password{}, fmt.Errorf("password must contain at least one uppercase letter")
	}
	if policy.RequireLower && !hasLower {
		return Password{}, fmt.Errorf("password must contain at least one lowercase letter")
	}
	if policy.RequireDigit && !hasDigit {
		return Password{}, fmt.Errorf("password must contain at least one digit")
	}
	if policy.RequireSpecial && !hasSpecial {
		return Password{}, fmt.Errorf("password must contain at least one special character")
	}

	return Password{plaintext: plaintext}, nil
}

func (p Password) Plaintext() string {
	return p.plaintext
}
