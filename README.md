<p>
  <img src="https://github.com/zaknesler/modulate/assets/7189795/527954fc-c0fc-4385-bb08-ab9d3a2b9191" alt="modulate logo" width="200">
</p>

I like keeping my favorite Spotify tracks in playlists labeled by year (e.g. a playlist called "2023") but it's annoying to add tracks to playlists manually.

This app allows you to transfer the tracks from your private "Liked" playlist to any other playlist you choose, basically treating it as a buffer so you can press ❤️ and go on with your obviously busy life. It also supports playlist-to-playlist transfers, and multiple watchers per user.

Once you connect your Spotify account and select a playlist, it'll stay running and auto-transfer your tracks every 60 minutes (this is configurable). It uses OAuth2 tokens and refreshes them when necessary, as well as a signed JWT stored as a cookie for browser authentication. It can handle any number of accounts, so you can host it for your friends or just keep it private.

<details>
  <summary><strong>View screenshot</strong></summary>
  <img src="https://github.com/zaknesler/modulate/assets/7189795/cedabcc3-8adb-41f7-92d7-c2fd81d07da1" alt="screenshot of app after configuring 2 watchers" width="400">
</details>

#### Usage

If you'd like to run it for yourself:

1. Create a [Spotify developer application](https://developer.spotify.com/dashboard)
1. Clone this repo
1. `cp .config/default.toml .config/local.toml`
1. Add Spotify creds to `.config/local.toml`
1. `touch .config/modulate.db`
1. `cargo run`
1. Go to [`localhost:4000`](http://localhost:4000), sign in, and configure

#### Thanks

A few honorable mentions:

- [rspotify](https://github.com/ramsayleung/rspotify) — for interacting w/ Spotify API
- [axum](https://github.com/tokio-rs/axum) — to provide a web interface
- [askama](https://github.com/djc/askama) — for HTML templating so we don't waste precious bytes with pointless JS
- [rusqlite](https://github.com/rusqlite/rusqlite) — for easily storing tokens and watchers
- [r2d2](https://github.com/sfackler/r2d2) — for managing the db connection
- you — I love you, let's get married please
