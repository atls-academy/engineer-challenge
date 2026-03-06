package redis

import (
	"context"
	"encoding/json"
	"errors"
	"time"

	"github.com/atrump/engineer-challenge/internal/domain"
	"github.com/redis/go-redis/v9"
)

const sessionKeyPrefix = "session:"

type RedisSessionRepository struct {
	client *redis.Client
}

func NewRedisSessionRepository(client *redis.Client) *RedisSessionRepository {
	return &RedisSessionRepository{
		client: client,
	}
}

func (r *RedisSessionRepository) Save(ctx context.Context, session *domain.Session) error {
	data, err := json.Marshal(session)
	if err != nil {
		return err
	}

	ttl := time.Until(session.ExpiresAt)
	if ttl <= 0 {
		return errors.New("session already expired")
	}

	return r.client.Set(ctx, sessionKeyPrefix+session.Token, data, ttl).Err()
}

func (r *RedisSessionRepository) FindByToken(ctx context.Context, token string) (*domain.Session, error) {
	data, err := r.client.Get(ctx, sessionKeyPrefix+token).Result()
	if err != nil {
		if errors.Is(err, redis.Nil) {
			return nil, nil // Not found
		}
		return nil, err
	}

	var session domain.Session
	if err := json.Unmarshal([]byte(data), &session); err != nil {
		return nil, err
	}

	return &session, nil
}

func (r *RedisSessionRepository) Delete(ctx context.Context, token string) error {
	return r.client.Del(ctx, sessionKeyPrefix+token).Err()
}
