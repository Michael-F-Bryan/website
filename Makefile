
docker: dist
	go get -t ./...
	go build ./cmd/website-server
	docker-compose build

install:
	go install ./cmd/...

dist:
	cd frontend && npm install && npm run build

.PHONY: install docker dist
