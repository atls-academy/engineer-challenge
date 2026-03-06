package valueobject

import "github.com/google/uuid"

type SessionID struct {
	value uuid.UUID
}

func NewSessionID() SessionID {
	return SessionID{value: uuid.New()}
}

func SessionIDFromString(s string) (SessionID, error) {
	id, err := uuid.Parse(s)
	if err != nil {
		return SessionID{}, err
	}
	return SessionID{value: id}, nil
}

func (s SessionID) String() string {
	return s.value.String()
}
