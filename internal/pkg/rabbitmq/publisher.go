package rabbitmq

import (
	"context"
	"encoding/json"
	"fmt"

	amqp "github.com/rabbitmq/amqp091-go"
)

const EmailQueueName = "email_queue"

type PasswordResetEvent struct {
	Email string `json:"email"`
	Token string `json:"token"`
}

type Publisher struct {
	conn *amqp.Connection
	ch   *amqp.Channel
}

// NewPublisher creates a new RabbitMQ publisher.
func NewPublisher(url string) (*Publisher, error) {
	conn, err := amqp.Dial(url)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to RabbitMQ: %w", err)
	}

	ch, err := conn.Channel()
	if err != nil {
		conn.Close()
		return nil, fmt.Errorf("failed to open a channel: %w", err)
	}

	_, err = ch.QueueDeclare(
		EmailQueueName,
		true,  // durable
		false, // delete when unused
		false, // exclusive
		false, // no-wait
		nil,   // arguments
	)
	if err != nil {
		ch.Close()
		conn.Close()
		return nil, fmt.Errorf("failed to declare a queue: %w", err)
	}

	return &Publisher{
		conn: conn,
		ch:   ch,
	}, nil
}

// Close closes the RabbitMQ connection and channel.
func (p *Publisher) Close() error {
	if p.ch != nil {
		p.ch.Close()
	}
	if p.conn != nil {
		return p.conn.Close()
	}
	return nil
}

// PublishPasswordReset publishes a password reset event to the email queue.
func (p *Publisher) PublishPasswordReset(ctx context.Context, email, token string) error {
	event := PasswordResetEvent{
		Email: email,
		Token: token,
	}

	body, err := json.Marshal(event)
	if err != nil {
		return fmt.Errorf("failed to marshal event: %w", err)
	}

	return p.ch.PublishWithContext(ctx,
		"",             // exchange
		EmailQueueName, // routing key
		false,          // mandatory
		false,          // immediate
		amqp.Publishing{
			ContentType: "application/json",
			Body:        body,
		})
}
