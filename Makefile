#!make

build:
	@buf generate
	@docker compose build blender

tauri-dev:
	@cd desktop-app && pnpm tauri dev

format:
	@cd desktop-app/src-tauri && cargo fmt --all
	@pre-commit run --all-files

.PHONY: format
.PHONY: build
