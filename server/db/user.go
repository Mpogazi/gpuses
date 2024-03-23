package db

import (
	"context"

	"go.mongodb.org/mongo-driver/mongo"
)

type User interface {
	CreateUser(ctx context.Context, email, password string) error
	GetUser(ctx context.Context, email string) error
	DeleteUser(ctx context.Context, email, password string) error
}

type UserDB struct {
	client            *mongo.Client
	AccountCollection *mongo.Collection
}

// CreateUser implements User.
func (u *UserDB) CreateUser(ctx context.Context, email string, password string) error {
	panic("unimplemented")
}

// DeleteUser implements User.
func (u *UserDB) DeleteUser(ctx context.Context, email string, password string) error {
	panic("unimplemented")
}

// GetUser implements User.
func (u *UserDB) GetUser(ctx context.Context, email string) error {
	panic("unimplemented")
}

func NewUser(client *mongo.Client) (User, error) {
	userDB := DatabaseFromClient(client, "user")
	userData := UserDB{
		client:            client,
		AccountCollection: userDB.Collection("account"),
	}

	return &userData, nil
}
