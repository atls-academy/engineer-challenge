package valueobject

import "github.com/google/uuid"

// UserID is a strongly-typed identifier for users.
type UserID struct {
	value uuid.UUID
}

func NewUserID() UserID {
	return UserID{value: uuid.New()}
}

func UserIDFromString(s string) (UserID, error) {
	id, err := uuid.Parse(s)
	if err != nil {
		return UserID{}, err
	}
	return UserID{value: id}, nil
}

func (u UserID) String() string {
	return u.value.String()
}

func (u UserID) IsZero() bool {
	return u.value == uuid.Nil
}

func (u UserID) Equals(other UserID) bool {
	return u.value == other.value
}
