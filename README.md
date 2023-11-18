### spotify sync

I like having my favorite Spotify tracks sorted by year, e.g. a playlist called *2023* but it's annoying to add tracks to playlists manually.

Basically, this app moves tracks from your private "Liked" playlist to any other playlist you choose, basically treating it as a buffer so you can press ❤️ and go on with your obviously busy life. Once you connect your Spotify account and select a playlist, it'll stay running and auto-transfer your tracks every night.

#### Usage

If you'd like to run a local version for yourself:

1. Create a [Spotify developer application](https://developer.spotify.com/dashboard)
2. Clone this repo
3. `cp .config/default.toml .config/local.toml`
4. Add Spotify creds to `.config/local.toml`
5. `cargo run`
6. Go to [`localhost:4000`](http://localhost:4000)

#### Thanks

This project utilizes the following:

- [rspotify](https://github.com/ramsayleung/rspotify) — for interacting w/ Spotify API
- [axum](https://github.com/tokio-rs/axum) — to provide a web API/interface
- [askama](https://github.com/djc/askama) — for HTML templating so we don't waste precious bytes with pointless JS
- you — I love you, let's get married please
