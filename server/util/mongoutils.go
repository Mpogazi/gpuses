package util

import (
	"sync"

	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"go.mongodb.org/mongo-driver/mongo/readpref"
)

var (
	SecondaryPreferredDatabaseOption = options.Database().SetReadPreference(readpref.SecondaryPreferred())

	globalDBOptsMu sync.RWMutex
	globalDBOpts   []*options.DatabaseOptions
)

func GloabalDatabaseOptions() []*options.DatabaseOptions {
	globalDBOptsMu.RLock()
	copiedOpts := make([]*options.DatabaseOptions, len(globalDBOpts))
	copy(copiedOpts, globalDBOpts)
	globalDBOptsMu.RUnlock()
	return copiedOpts
}

func AddGlobalDatabaseOptions(opts ...*options.DatabaseOptions) {
	newOpts := make([]*options.DatabaseOptions, len(opts))
	copy(newOpts, opts)
	globalDBOptsMu.Lock()
	globalDBOpts = newOpts
	globalDBOptsMu.Unlock()
}

func DatabaseFromClient(client *mongo.Client, dbName string, opts ...*options.DatabaseOptions) *mongo.Database {
	var allOpts []*options.DatabaseOptions
	allOpts = append(allOpts, GloabalDatabaseOptions()...)
	allOpts = append(allOpts, opts...)
	return client.Database(dbName, allOpts...)
}
