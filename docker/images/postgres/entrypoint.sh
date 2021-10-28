#!/bin/sh

install_pgtap() {
  until pg_isready
  do
    echo "Waiting for postgres to start"
    sleep 1
  done

  echo "Installing pgTAP"
  cd /pgtap && make install && make installcheck
  psql -d $POSTGRES_DB -c "create extension if not exists pgtap"
}

install_pgtap &
docker-entrypoint.sh postgres
