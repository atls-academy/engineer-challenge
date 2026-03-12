package postgres

import (
	"github.com/jmoiron/sqlx"
)

type PostgresRepo struct {
	db *sqlx.DB
}

func NewPostgres(db *sqlx.DB) *PostgresRepo {
	return &PostgresRepo{db: db}
}
