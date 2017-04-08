dist_zip := $(shell mktemp -u).zip

zip: clean
	zip -v -r $(dist_zip) .
	@echo
	@echo
	@echo "Zip file is available at $(dist_zip)"

clean:
	find . -name '*.pyc' -delete
	find . -name '__pycache__' -delete

.PHONY: zip clean
