#!make

gen:
	@buf generate
	@sed 's/from cf3d\.v1/from cloudforge3d.client.cf3d.v1/g' blender-extension/cloudforge3d/client/cf3d/v1/jobs_pb2_grpc.py > temp_file && mv temp_file blender-extension/cloudforge3d/client/cf3d/v1/jobs_pb2_grpc.py

build:
	@docker compose build blender

tauri-dev:
	@cd desktop-app && pnpm tauri dev

format:
	@cd desktop-app/src-tauri && cargo fmt --all
	@pre-commit run --all-files
	@buf format -w

.PHONY: format
.PHONY: build
