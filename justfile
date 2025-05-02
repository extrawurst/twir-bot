
dotenv:
    direnv allow

run:
	cargo r --bin twir-bot

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
