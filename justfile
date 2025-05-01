DISCORD_TOKEN:=""

run:
	DISCORD_TOKEN={{DISCORD_TOKEN}} CHANNEL_ID=904895263955644446 cargo r --bin twir-bot

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
