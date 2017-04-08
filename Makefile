dist_zip := $(shell mktemp -u).zip
PYTHON := python3
UNIT_FILE := website.service
UNIT_DIR := /etc/systemd/system
NGINX_DIR := /etc/nginx
SITE := michaelfbryan.com
dependencies := django gunicorn djangorestframework

install:
	pip3 install --upgrade $(dependencies)

update:
	git pull --rebase
	$(PYTHON) manage.py makemigrations
	$(PYTHON) manage.py migrate

reload: 
	echo 'yes' | $(PYTHON) manage.py collectstatic
	sudo cp "$(UNIT_FILE)" "$(UNIT_DIR)"
	sudo cp "$(SITE)" "$(NGINX_DIR)/sites-available/$(SITE)"
	sudo ln --symbolic --force "$(NGINX_DIR)/sites-available/$(SITE)" "$(NGINX_DIR)/sites-enabled/$(SITE)"
	sudo systemctl daemon-reload
	sudo systemctl restart nginx
	sudo systemctl stop "$(UNIT_FILE)"
	sudo systemctl start "$(UNIT_FILE)"

zip: clean
	zip -v -r "$(dist_zip)" .
	@echo
	@echo
	@echo "Zip file is available at $(dist_zip)"

clean:
	find . -name '*.pyc' -delete
	find . -name '__pycache__' -delete

.PHONY: zip clean install reload
