package event

import "time"

type DomainEvent interface {
	EventName() string
	OccurredAt() time.Time
	AggregateID() string
}

type BaseEvent struct {
	Name        string    `json:"event_name"`
	Timestamp   time.Time `json:"occurred_at"`
	AggregateId string    `json:"aggregate_id"`
}

func (e BaseEvent) EventName() string     { return e.Name }
func (e BaseEvent) OccurredAt() time.Time { return e.Timestamp }
func (e BaseEvent) AggregateID() string   { return e.AggregateId }

type UserAuthenticated struct {
	BaseEvent
	UserID string `json:"user_id"`
}

func NewUserAuthenticated(sessionID, userID string) UserAuthenticated {
	return UserAuthenticated{
		BaseEvent: BaseEvent{
			Name:        "auth.user.authenticated",
			Timestamp:   time.Now(),
			AggregateId: sessionID,
		},
		UserID: userID,
	}
}

type SessionRevoked struct {
	BaseEvent
}

func NewSessionRevoked(sessionID string) SessionRevoked {
	return SessionRevoked{
		BaseEvent: BaseEvent{
			Name:        "auth.session.revoked",
			Timestamp:   time.Now(),
			AggregateId: sessionID,
		},
	}
}
