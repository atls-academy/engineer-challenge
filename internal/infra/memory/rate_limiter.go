package memory

import (
	"context"
	"sync"
	"time"
)

// RateLimiter is a simple in-memory thread-safe rate limiter.
type RateLimiter struct {
	mu      sync.Mutex
	entries map[string]*rateLimitEntry
	quit    chan struct{}
}

type rateLimitEntry struct {
	count     int
	expiresAt time.Time
}

// NewRateLimiter creates a new in-memory rate limiter and starts a background cleanup routine.
func NewRateLimiter() *RateLimiter {
	limiter := &RateLimiter{
		entries: make(map[string]*rateLimitEntry),
		quit:    make(chan struct{}),
	}
	
	go limiter.cleanupLoop()
	return limiter
}

// Allow tracks the number of times this key was seen within the current window and returns
// true if the limit has not been exceeded.
func (l *RateLimiter) Allow(ctx context.Context, key string, limit int, window time.Duration) (bool, error) {
	l.mu.Lock()
	defer l.mu.Unlock()

	now := time.Now()

	entry, exists := l.entries[key]

	if !exists || now.After(entry.expiresAt) {
		l.entries[key] = &rateLimitEntry{
			count:     1,
			expiresAt: now.Add(window),
		}
		return 1 <= limit, nil
	}

	entry.count++
	return entry.count <= limit, nil
}

// Close stops the background cleanup loop.
func (l *RateLimiter) Close() {
	close(l.quit)
}

// cleanupLoop periodically removes expired keys to prevent memory leaks.
func (l *RateLimiter) cleanupLoop() {
	ticker := time.NewTicker(1 * time.Minute)
	defer ticker.Stop()

	for {
		select {
		case <-l.quit:
			return
		case <-ticker.C:
			l.mu.Lock()
			now := time.Now()
			for key, entry := range l.entries {
				if now.After(entry.expiresAt) {
					delete(l.entries, key)
				}
			}
			l.mu.Unlock()
		}
	}
}
