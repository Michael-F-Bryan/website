dist_zip := $(shell mktemp -u).zip
PYTHON := python3
UNIT_FILE := website.service
UNIT_DIR := /etc/systemd/system
NGINX_DIR := /etc/nginx
SITE := michaelfbryan.com
dependencies := django gunicorn djangorestframework

help:
	@echo "Makefile for automating various jobs on the website."
	@echo
	@echo "Targets:"
	@echo "--------"
	@echo "upgrade        Upgrade the entire system, doing everything"
	@echo "                 necessary to get it running"
	@echo "install        Install all necessary dependencies"
	@echo "update         Grab the latest version from github and run any"
	@echo "                 necessary migrations"
	@echo "reload         Reload the nginx config and systemd unit file"
	@echo "clean          Get rid of any unnecessary crap"
	@echo "help           Print this help text"

upgrade: install update reload

install:
	pip3 install $(dependencies)

update:
	git pull --rebase
	$(PYTHON) manage.py makemigrations
	$(PYTHON) manage.py migrate

reload: 
	$(PYTHON) manage.py collectstatic -v0 --no-input
	sudo cp "$(UNIT_FILE)" "$(UNIT_DIR)"
	sudo cp "$(SITE)" "$(NGINX_DIR)/sites-available/$(SITE)"
	sudo ln --symbolic --force "$(NGINX_DIR)/sites-available/$(SITE)" "$(NGINX_DIR)/sites-enabled/$(SITE)"
	sudo systemctl daemon-reload
	sudo systemctl restart nginx
	sudo systemctl stop "$(UNIT_FILE)"
	sudo systemctl start "$(UNIT_FILE)"

clean:
	find . -name '*.pyc' -delete
	find . -name '__pycache__' -delete
	$(RM) ./static

.PHONY: help upgrade install update reload clean
