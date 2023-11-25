up:
	docker compose up -d --build --force-recreate --remove-orphans

down:
	docker compose down

run_ui:
	cd ui && pnpm run dev || cd -
