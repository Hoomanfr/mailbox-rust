include .env.local
export

createlocaldbcontainer:
	docker run -d --name postgres-service -e POSTGRES_PASSWORD=example -p 5432:5432 postgres

createlocalmailboxdb:
	sqlx database create

droplocalmailboxdb:
	sqlx database drop

migrateadd:
	sqlx migrate add $(name)

migrateup:
	sqlx migrate run

migratedown:
	sqlx migrate revert

test:
	cargo test

run:
	cargo run

dockerimage:
	docker build -t mailbox .

dockerrun:
	docker network create mailbox-network || true
	docker network connect mailbox-network postgres-service || true
	docker run --env-file .env.docker --network=mailbox-network -d --name mailbox-service -p 3000:3000 mailbox

dockerbuildandrun:
	make dockerimage
	make dockerrun