start-client:
	cd website && pnpm run dev

start-server:
	cd server && go run cmd/main.go

start-mongo-server:
	@echo "Starting mongo server"
	docker run --name gpuse -p 27017:27017 -e MONGO_INITDB_ROOT_USERNAME=root -e MONGO_INITDB_ROOT_PASSWORD=gpuse_secret -d mongo:7.0.0

stop-mongo-server:
	@echo "Stopping mongo server"
	docker stop gpuse

format-go:
	go fmt ./...