TEST_SQL_PATH ?= tests/sql/*.sql

test_sql:
	@ pg_prove -U timer -h localhost -p 5555 -d timer $(TEST_SQL_PATH)

docker_start:
	@ docker compose up -d

docker_restart:
	@ docker compose down && docker compose up -d

migrate_db:
	@ sqlx migrate run

x:
	@ echo ${X:-x}
