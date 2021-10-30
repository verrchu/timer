TEST_SQL_PATH ?= tests/sql/*.sql
DB_PORT ?= 25432

test_sql:
	@ pg_prove -U timer -h localhost -p $(DB_PORT) -d timer $(TEST_SQL_PATH)

docker_start:
	@ docker compose up -d

docker_restart:
	@ docker compose down && docker compose up -d

migrate_db:
	@ sqlx migrate run

x:
	@ echo ${X:-x}
