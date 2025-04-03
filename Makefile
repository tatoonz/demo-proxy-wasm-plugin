# A Makefile is not required, but can be helpful for organizing the actions
# needed when working on a project.

.PHONY: help build start

.DEFAULT_GOAL := help

help: 	## Display this help message.
	@echo "Please use \`make <target>' where <target> is one of:"
	@awk -F ':.*?## ' '/^[a-zA-Z]/ && NF==2 {printf "  %-20s %s\n", $$1, $$2}' $(MAKEFILE_LIST) | sort

build: 
	cd my-first-plugin && cargo build --release

start: build
	docker compose up -d

stop:
	docker compose down