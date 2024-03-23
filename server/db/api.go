package db

import "context"

type Database interface {
	UserDatabase
}

type UserDatabase interface {
	UserByEmail(ctx context.Context, email string) (*User, error)
	UserByID(ctx context.Context, id string) (*User, error)
}