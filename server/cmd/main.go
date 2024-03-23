package main

import (
	"context"
	"log"

	"github.com/Mpogazi/gpuses/db"
)

func main() {
	connectToDB()
}

func connectToDB() {
	client, err := db.ConnectToMongoDB()
	if err != nil {
		log.Fatal(err)
	}
	log.Println("Connected to MongoDB")
	defer client.Disconnect(context.Background())
}