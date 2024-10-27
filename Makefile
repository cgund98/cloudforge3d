#!make

build:
	@buf generate
	@docker compose build blender

tauri-dev:
	@cd desktop-app && pnpm tauri dev

format:
	@cd desktop-app/src-tauri && cargo fmt --all
	@pre-commit run --all-files
	@buf format -w

.PHONY: format
.PHONY: build
