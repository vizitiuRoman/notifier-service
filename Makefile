docker-build:
	docker build -t vizitiuroman/notifier-service --no-cache -f ./docker/Dockerfile .
	docker tag vizitiuroman/notifier-service vizitiuroman/notifier-service:1.0.0

docker-publish:
	docker build -t vizitiuroman/notifier-service --no-cache -f ./docker/Dockerfile .
	docker tag vizitiuroman/notifier-service vizitiuroman/notifier-service:1.0.0
	docker push vizitiuroman/notifier-service:1.0.0

watch:
	cargo-watch -x 'run src/main.rs'

lint:
	cargo fmt --all -- --check

test:
	cargo test --verbose
