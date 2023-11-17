### spotify sync

I like keeping my "Liked" Spotify tracks in a playlist organized by year, e.g. a playlist called *2023*, but I can't do that if I’m driving or don't want to open the app.

This thing simply watches your private “Liked” playlist and adds any tracks to a playlist you select (it syncs them at 11:55pm that night), then removes them from your "Liked" playlist.

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

- rspotify — for interacting w/ Spotify API
- axum — to provide a web API/interface
- askama — for HTML templating so we don't waste precious bytes with pointless JS
- you — I love you, let's get married please
