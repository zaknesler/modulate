pub mod client;
pub mod error;
pub mod id;
pub mod model;
pub mod response;
pub mod token;
pub mod util;

/// Spotify URL for a user's "Liked Tracks" playlist.
pub const SPOTIFY_LIKED_TRACKS_URL: &str = "https://open.spotify.com/collection/tracks";
