web: ROCKET_PORT=$PORT ROCKET_DATABASES={db={url=$DATABASE_URL}} ./target/release/laenorweb2
release: ./target/release/diesel migration run
