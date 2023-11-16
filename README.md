### spotify sync

I like keeping my "Liked" Spotify tracks in a playlist organized by year, e.g. a playlist called *2023*, but I can't do that if I’m driving or don't want to open the app.

This thing simply watches your private “Liked” playlist and adds any tracks to a playlist you select (it syncs them at 11:55pm that night), then removes them from your "Liked" playlist.

#### Usage

You could just use the instance I run for myself ([spotify.zak.bz](https://spotify.zak.bz)), I don't mind. If you'd like to run it yourself:

1. clone it
2. make Spotify developer app
3. add creds to `config.toml`
4. `cargo run` or `cargo build`
5. visit [`localhost:4000`](http://localhost:4000)

#### Thanks

This project utilizes the following:

- rspotify — for interacting w/ Spotify API
- axum — to provide a web API/interface
- askama — for HTML templating so we don't waste precious bytes with pointless JS
- you — I love you, let's get married please
