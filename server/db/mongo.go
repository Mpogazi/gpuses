package db

import (
	"context"

	"github.com/Mpogazi/gpuses/util"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

func ConnectToMongoDB(client *mongo.Client, dbName string) (*mongo.Database, error) {
	config, err := util.LoadConfig(".")
	if err != nil {
		return nil, err
	}
	mongoURI := config.MongoURI

	serverAPI := options.ServerAPI(options.ServerAPIVersion1)
	opts := options.Client().ApplyURI(mongoURI).SetServerAPIOptions(serverAPI)

	client, err := mongo.Connect(context.TODO(), opts)
	if err != nil {
		return nil, err
	}

	return client, nil
}
