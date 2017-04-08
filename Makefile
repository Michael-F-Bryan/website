dist_zip := $(shell mktemp -u).zip
PYTHON := python3
UNIT_FILE := website.service
UNIT_DIR := /etc/systemd/system

install:
	git pull --rebase --autostash
	pip install --user --upgrade django
	pip install --user --upgrade gunicorn
	$(PYTHON) manage.py makemigrations
	$(PYTHON) manage.py migrate

reload:
	sudo cp "$(UNIT_FILE)" "$(UNIT_DIR)"
	sudo systemctl daemon-reload
	sudo systemctl restart "$(UNIT_FILE)"

zip: clean
	zip -v -r "$(dist_zip)" .
	@echo
	@echo
	@echo "Zip file is available at $(dist_zip)"

clean:
	find . -name '*.pyc' -delete
	find . -name '__pycache__' -delete

.PHONY: zip clean install
