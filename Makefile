#!make

build:
	@buf generate
	@docker compose build blender

.PHONY: format
.PHONY: build
