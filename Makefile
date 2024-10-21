#!make

build:
	@buf generate
	@docker compose build blender

tauri-dev:
	@cd desktop-app && pnpm tauri dev

.PHONY: format
.PHONY: build
