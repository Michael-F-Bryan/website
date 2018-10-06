
docker: dist
	go get -t ./...
	CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o website-server ./cmd/website-server
	chmod +x website-server
	docker-compose build

install:
	go install ./cmd/...

dist:
	cd frontend && npm run build

clean:
	$(RM) website-server
	cd frontend && $(RM) -r node_modules

.PHONY: install docker dist clean
