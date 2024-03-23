package db

import (
	"context"

	"github.com/Mpogazi/gpuses/util"
	"go.mongodb.org/mongo-driver/mongo"
)

var DatabaseFromClient = util.DatabaseFromClient

func InTransaction(ctx context.Context, client *mongo.Client, callback func(sessionContext context.Context) error) error {
	err := client.UseSession(ctx, func(sc mongo.SessionContext) error {
		err := sc.StartTransaction()
		if err != nil {
			return err
		}

		err = callback(mongo.NewSessionContext(ctx, sc))
		if err != nil {
			abortErr := sc.AbortTransaction(sc)
			if abortErr != nil {
				return abortErr
			}
			return err
		}

		return sc.CommitTransaction(sc)
	})
	return err
}
