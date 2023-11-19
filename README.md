<img src="https://github.com/zaknesler/spotify-sync/assets/7189795/5720db41-ea10-4b60-80a6-ca66d1858e9b" alt="spotify sync logo" width="200">

I like having my favorite Spotify tracks sorted by year, e.g. a playlist called *2023*, but it's annoying to add tracks to playlists manually.

This app moves tracks from your private "Liked" playlist to any other playlist you choose, basically treating it as a buffer so you can press ❤️ and go on with your obviously busy life. Once you connect your Spotify account and select a playlist, it'll stay running and auto-transfer your tracks every hour.

<details>
  <summary>View screenshots</summary>

  <img src="https://github.com/zaknesler/spotify-sync/assets/7189795/aa849b2f-d970-45e3-80b2-3221b23b8a63" alt="app screenshot" width="350">
  <br>
  <img src="https://github.com/zaknesler/spotify-sync/assets/7189795/641f4251-1fed-463b-8c34-3d2b4fb1d001" alt="app screenshot" width="350">
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
