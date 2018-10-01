

install:
	go install ./cmd/...

dist:
	cd frontend && npm run build

.PHONY: install
