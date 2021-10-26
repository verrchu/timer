test_sql:
	@ pg_prove -U timer -h localhost -p 5555 -d timer tests/sql/*.sql
