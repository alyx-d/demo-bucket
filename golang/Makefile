build:
	@go build -o bin/ecom cmd/main.go


test:
	@go test -v ./...

run: build
	@./bin/ecom
clean:
	@rm -rf bin

migration:
	@migrate create -ext sql -dir cmd/migrate/migrations $(filter-out $@, $(MAKECMDGOALS))

migrate-up:
	@go run cmd/migrate/main.go up

migrate-down:
	@go run cmd/migrate/main.go down