.PHONY: all fmt test build clean check

all: fmt check test build

fmt:
	go fmt ./...

test:
	go test -v -race ./...

build:
	go build ./...

clean:
	go clean
	rm -f coverage.out

check:
	go vet ./...

coverage:
	go test -race -coverprofile=coverage.out ./...
	go tool cover -html=coverage.out
