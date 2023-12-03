<p>
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/zaknesler/modulate/assets/7189795/1c5f53fc-d014-4e7b-8c61-7122dedb7445">
    <source media="(prefers-color-scheme: light)" srcset="https://github.com/zaknesler/modulate/assets/7189795/1e6e73b4-2be5-40cb-9a59-d1983c4d5448">
    <img src="https://github.com/zaknesler/modulate/assets/7189795/1e6e73b4-2be5-40cb-9a59-d1983c4d5448" alt="modulate logo" width="150">
  </picture>
</p>

I like keeping my favorite Spotify tracks in playlists labeled by year (e.g. a playlist called "2023") but it's annoying to add tracks to playlists manually.

This tool allows you to transfer the tracks from one playlist to another on an interval. Most importantly, this also includes moving tracks from your liked/saved playlist, so you can press ❤️ and go on with your obviously busy life.

Once you connect your Spotify account and configure a watcher, it'll stay running and auto-transfer your tracks on the interval you choose (e.g. every hour/week/day). You can also configure it to just copy tracks instead of removing them. For example, you can keep your saved tracks synced to a collaborative playlist with your friends, or vice-versa.

<details>
  <summary><strong>View screenshot</strong></summary>
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/zaknesler/modulate/assets/7189795/348cd29f-878b-4db7-b140-532e35412afd">
    <source media="(prefers-color-scheme: light)" srcset="https://github.com/zaknesler/modulate/assets/7189795/872f7451-85bf-4c30-b8cf-c1f6cf951d81">
    <img src="https://github.com/zaknesler/modulate/assets/7189795/872f7451-85bf-4c30-b8cf-c1f6cf951d81" alt="screenshot of app after configuring watchers" width="400">
  </picture>
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
