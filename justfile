set dotenv-load

init:
	cargo install cargo-watch
	cargo install sqlx-cli
	
	sqlx database create
	just db-migrate

dev-server:
	cargo watch -w src -w templates -w tailwind.config.js -w input.css -x run

#dev-check:
#cargo check --quiet --workspace --message-format=json --all-targets --keep-going

dev-srv:
	cargo watch --clear --no-restart -w src -w templates -w tailwind.config.js -w input.css -x run

dev-tailwind:
	./tailwindcss/tailwindcss -i input.css -o assets/output.css --watch=always
#	./tailwindcss/tailwindcss input.css assets/output.css --watch=always
build-server:
	cargo build --release

build-tailwind:
	./tailwindcss -i input.css -o assets/output.css --minify


db-migrate:
  echo "Migrating ..."
  sqlx migrate run --source $MIGRATIONS_PATH;

db-reset:
  echo "Resetting ..."
  sqlx database drop && sqlx database create && sqlx migrate run --source $MIGRATIONS_PATH
  sqlite3 $DATABASE_PATH < seeds/seed-users.sql

dev:
	#!/bin/sh
	pid1=$!
	echo $pid1
	just dev-server
	pid2=$!
	trap "kill $pid1 $pid2" EXIT
	wait $pid1 $pid2
# open http://0.0.0.0:3001

dev-debug:
	#!/bin/sh
	pid1=$!
	RUST_BACKTRACE=1 just dev-watch-debug &
	pid2=$!
	trap "kill $pid1 $pid2" EXIT
	wait $pid1 $pid2
# open http://0.0.0.0:3000

clean:
    cargo update -vv
    cargo clean
    cargo build --release

commit:
    git add .
    git commit -m "Add new feature"
