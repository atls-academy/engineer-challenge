package event

import "time"

// DomainEvent is the base interface for all domain events.
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

type UserRegistered struct {
	BaseEvent
	Email string `json:"email"`
}

func NewUserRegistered(userID, email string) UserRegistered {
	return UserRegistered{
		BaseEvent: BaseEvent{
			Name:        "identity.user.registered",
			Timestamp:   time.Now(),
			AggregateId: userID,
		},
		Email: email,
	}
}

type PasswordResetRequested struct {
	BaseEvent
	Email string `json:"email"`
}

func NewPasswordResetRequested(userID, email string) PasswordResetRequested {
	return PasswordResetRequested{
		BaseEvent: BaseEvent{
			Name:        "identity.password_reset.requested",
			Timestamp:   time.Now(),
			AggregateId: userID,
		},
		Email: email,
	}
}

type PasswordResetCompleted struct {
	BaseEvent
}

func NewPasswordResetCompleted(userID string) PasswordResetCompleted {
	return PasswordResetCompleted{
		BaseEvent: BaseEvent{
			Name:        "identity.password_reset.completed",
			Timestamp:   time.Now(),
			AggregateId: userID,
		},
	}
}
