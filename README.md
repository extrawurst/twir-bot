# twir-bot
Little discord 🤖 helping [TWIR](https://github.com/rust-lang/this-week-in-rust) 🦀 authors managing weekly community updates.

# context
The weekly updates from the community is assembled by a couple of contributors gathering links in a discord channel first for review and later assembly into a PR. Assembling the PR usually meant a lot of manual effort. This bot simplifies this a lot:

![demo](demo.gif)

# TODO
* [x] limit bot to specific channel
* [ ] do dupe detection on `!collect`
* [ ] language tag detection on collection (+ reaction)
* [ ] on each new msg perform check if it would be added (+ reaction)
* [ ] allow emojis to classify content (official, project updates...)
* [ ] detect youtube content and mark as `[video]`
* [x] scrape reddit for new links posted using [roux](https://github.com/halcyonnouveau/roux)
* [x] scrape lobsters for new links posted
* [ ] scrape hackernews (harder because no categories)
* [ ] scrape dev.to
* [ ] dedup scrape results and post them for review
