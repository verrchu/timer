db:
	@ docker run -d -p $(PORT):5432 \
		-e POSTGRES_PASSWORD=pg \
		postgres:14.0
