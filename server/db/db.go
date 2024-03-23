package db

import (
	"context"
	"errors"

	"go.mongodb.org/mongo-driver/mongo"
	"go.opencensus.io/trace"
)

type mongoData struct {
	userCollection *mongo.Collection

	client *mongo.Client
}

// DeleteUser implements Database.
func (m *mongoData) DeleteUser(ctx context.Context, id string) error {
	panic("unimplemented")
}

// UserByEmail implements Database.
func (m *mongoData) UserByEmail(ctx context.Context, email string) (*User, error) {
	panic("unimplemented")
}

// UserByID implements Database.
func (m *mongoData) UserByID(ctx context.Context, id string) (*User, error) {
	panic("unimplemented")
}

var (
	DBName = "main"

	CreatedOnField   = "created_on"
	UpdatedOnField   = "updated_on"
	DeletedFiledName = "deleted"
)

func MewMongoDB(ctx context.Context, client *mongo.Client) (Database, error) {
	return newMongoDB(ctx, client)
}

func newMongoDB(ctx context.Context, client *mongo.Client) (Database, error) {
	var db mongoData

	if client == nil {
		return &db, errors.New("mongo client cannot be nil")
	}

	db.client = client

	databaseDB := DatabaseFromClient(client, DBName)

	db.userCollection = databaseDB.Collection("user")

	ctx, span := trace.StartSpan(ctx, "Database::NewMongoDB")
	defer span.End()

	return NewTrace("mongodb", &db), nil
}
