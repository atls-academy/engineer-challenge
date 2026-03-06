package valueobject

import "time"

// TokenPair holds an access and refresh token pair.
type TokenPair struct {
	AccessToken  string
	RefreshToken string
	ExpiresAt    time.Time
}
