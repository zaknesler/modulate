<img src="https://github.com/zaknesler/spotify-sync/assets/7189795/60f2d659-9ff4-402e-ac64-0df820b0fa8d" alt="spotify sync logo" width="200">

I prefer having my favorite Spotify tracks in playlists sorted by year (e.g. a playlist called "2023") but it's annoying to add tracks to playlists manually.

This app transfers tracks from your private "Liked" playlist to any other playlist you choose, basically treating it as a buffer so you can press ❤️ and go on with your obviously busy life. Once you connect your Spotify account and select a playlist, it'll stay running and auto-transfer your tracks every hour (configurable).

<details>
  <summary>View screenshots</summary>
  <img src="https://github.com/zaknesler/spotify-sync/assets/7189795/e6090ac5-c0d1-4a59-b8af-5ced994dd022" alt="screenshot before configuring watcher" width="400">
  <br>
  <img src="https://github.com/zaknesler/spotify-sync/assets/7189795/44f79cf6-d980-4b57-b55f-9a02156b8fee" alt="screenshot after configuring watcher" width="400">
</details>

#### Usage

If you'd like to run it for yourself:

1. Create a [Spotify developer application](https://developer.spotify.com/dashboard)
1. Clone this repo
1. `cp .config/default.toml .config/local.toml`
1. Add Spotify creds to `.config/local.toml`
1. `touch .config/sync.db`
1. `cargo run`
1. Go to [`localhost:4000`](http://localhost:4000) and follow setup

#### Thanks

This project utilizes the following:

- [rspotify](https://github.com/ramsayleung/rspotify) — for interacting w/ Spotify API
- [axum](https://github.com/tokio-rs/axum) — to provide a web API/interface
- [askama](https://github.com/djc/askama) — for HTML templating so we don't waste precious bytes with pointless JS
- you — I love you, let's get married please
