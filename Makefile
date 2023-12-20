DISCORD_TOKEN=1234

run:
	DISCORD_TOKEN=${DISCORD_TOKEN} cargo r

check:
	cargo fmt -- --check
	cargo sort -c -w
	cargo c
	cargo clippy
	cargo t

scrape-lobsters:
	cargo r -p=scrape_lobsters

scrape-reddit:
	cargo r -p=scrape_reddit