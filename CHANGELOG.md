# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

Nothing yet.

## [0.14.1] - 2023-12-16

### Changed

- Return user fields when upserting token
- Improved client API

## [0.14.0] - 2023-12-15

### Added

- Store transfer results in `transfers` table

### Changed

- Add config option for check interval, and changed default to 5 minutes
- Handle Spotify errors more gracefully
- Handle `429 Too Many Requests` response that doesn't return JSON

### Fixed

- Playlist track remove endpoint passing incorrect body

## [0.13.0] - 2023-12-10

### Added

- `User` model to improve readability of user repo results
- `last_sync_at` date to `Watcher` model
- (Optional) logging to `Sentry`

### Changed

- Refactored transfer logic to make it more *modular*
- Callback URL/CORS origin are now unified into one `WEB_PUBLIC_URL` env variable
- Web error responses are handled a bit more nicely
- Input font size is now `1rem` (16px) on mobile to prevent auto-zooming on focus

### Fixed

- `dashboard.html` template formatting (accidentally mutilated by prettier)

## [0.12.0] - 2023-12-09

### Changed

- Removed `lazy_static` in favor of keeping config in the context (like it was before)
- Switched configuration parsing crate from `config` to `figment`
- Moved from .toml configuration files to `.env`

### Fixed

- Don't duplicate query params when making paginated requests
- Access tokens not refreshing on sync thread

## [0.11.0] - 2023-12-08

### Changed

- Removed `rspotify` in favor of using the standard `oauth2` crate directly
  - i.e. now we're making request to Spotify manually with `reqwest`, for more control
- Organized modules a bit better and split error enums
- Spotify errors are now handled a bit better
- `playlist_from` is verified to exist when creating watcher
- Renamed internal `user_id` to `user_uri` to be more accurate

## [0.10.1] - 2023-12-04

### Fixed

- Input colors on dark mode

### Changed

- First input/select autofocuses
- Input now re-focuses on mode change

## [0.10.0] - 2023-12-03

### Added

- Watchers can now be created for any playlist (by manually entering URI/URL)

### Changed

- If tracks cannot be removed, a custom error is thrown

## [0.9.0] - 2023-11-30

### Changed

- Intervals are now attached to each watcher
  - Supports running watcher every hour, day, or week
- Auth middleware will now redirect unset invalid JWTs and redirect to connect page

## [0.8.0] - 2023-11-29

### Changed

- Updated to axum `v0.7` (as well as `tower-http`, `tower-cookies`, and `askama-axum`)
- Reverse items when inserting to retain original sort order

## [0.7.2] - 2023-11-27

### Changed

- Minor style tweaks
- Show logo and intro on connect page

## [0.7.1] - 2023-11-27

### Added

- Dark mode support

### Changed

- Make watcher validation response text more human-friendly

## [0.7.0] - 2023-11-26

### Fixed

- Playlist images will now display the only option regardless of width

### Changed

- Endpoints are now called via `fetch` using JSON, to make displaying errors easier
- Errors are now displayed after API calls

## [0.6.0] - 2023-11-26

### Added

- Ability to disable syncing with `sync.enabled` (set to `true` by default, but `false` will disable)

### Changed

- Env prefix `SPOTIFY` has been changed to `MODULATE`
- Repo methods will now return `Option` instead of erroring if a record could not be found (not including ones that return arrays)
- Can no longer create a watcher if either:
  1. A watcher with `should_remove` enabled already exists for that playlist
  2. They are trying to create a watcher with `should_remove` enabled and *any* other watcher already exists for that playlist

### Fixed

- Env arg parsing, e.g. `MODULATE_SYNC_ENABLED=false` will disable syncing

## [0.5.5] - 2023-11-22

### Changed

- Minor copy tweaks
- Created `Session` struct to store `user_id`, `token`, and `client`
- Pass entire `Watcher` to `PlaylistTransfer::transfer` instead of individual fields
- Don't include `user_token` in `Watcher`

## [0.5.4] - 2023-11-21

### Fixed

- Empty playlists were showing with the Liked Tracks heart icon, but will now show with generic music icon

### Changed

- Update font to Inter
- Use `1rem` padding for smaller screens (instead of `2rem`)
- Display logo and version number

## [0.5.3] - 2023-11-21

### Changed

- Display playlist images
- Link to playlists
- Some more style tweaks
- Add Spotify attribution

## [0.5.2] - 2023-11-21

### Changed

- Updated logo
- Some style tweaks

## [0.5.1] - 2023-11-20

### Added

- Ability for a user to delete their account
- Text to denote which watchers will remove tracks from original playlist

## [0.5.0] - 2023-11-20

### Changed

- Added checkbox to enable/disable removing items from original playlist
- CSS file is now stored separately and included in the base template

## [0.4.0] - 2023-11-20

### Added

- Multiple watchers can now be created (and synced or removed)
- Playlist-playlist watchers can now be created

### Changed

- Added better abstraction for transferring tracks between playlists
- Renamed project from `spotify-sync` to `modulate` (a cooler name, but also required by Spotify developer terms)

## [0.3.0] - 2023-11-19

### Added

- Added button to instantly sync tracks

## [0.2.0] - 2023-11-19

### Changed

- Make config a static global with `lazy_static`
- Make interval configurable
- Any tracks that already exist in the playlist will not be added (fixes #2)

## [0.1.3] - 2023-11-19

### Changed

- Minor style improvements

## [0.1.2] - 2023-11-19

### Changed

- Validate form data sent to `POST /watcher`
- Disable create button until playlist is selected

## [0.1.1] - 2023-11-19

### Changed

- Don't fetch playlists if user already has a watcher configured

### Fixed

- Fix font sizes on buttons/inputs

## [0.1.0] - 2023-11-19

### Added

- Initial release
- Basic functionality implemented:
  - Sign in with Spotify
  - Create a "watcher" for a single playlist
  - Background task runs on an interval, transferring tracks for each user that has configured a "watcher"

[Unreleased]: https://github.com/zaknesler/modulate/compare/v0.14.1...HEAD
[0.14.1]: https://github.com/zaknesler/modulate/compare/v0.14.0...v0.14.1
[0.14.0]: https://github.com/zaknesler/modulate/compare/v0.13.0...v0.14.0
[0.13.0]: https://github.com/zaknesler/modulate/compare/v0.12.0...v0.13.0
[0.12.0]: https://github.com/zaknesler/modulate/compare/v0.11.0...v0.12.0
[0.11.0]: https://github.com/zaknesler/modulate/compare/v0.10.1...v0.11.0
[0.10.1]: https://github.com/zaknesler/modulate/compare/v0.10.0...v0.10.1
[0.10.0]: https://github.com/zaknesler/modulate/compare/v0.9.0...v0.10.0
[0.9.0]: https://github.com/zaknesler/modulate/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/zaknesler/modulate/compare/v0.7.2...v0.8.0
[0.7.2]: https://github.com/zaknesler/modulate/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/zaknesler/modulate/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/zaknesler/modulate/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/zaknesler/modulate/compare/v0.5.5...v0.6.0
[0.5.5]: https://github.com/zaknesler/modulate/compare/v0.5.4...v0.5.5
[0.5.4]: https://github.com/zaknesler/modulate/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/zaknesler/modulate/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/zaknesler/modulate/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/zaknesler/modulate/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/zaknesler/modulate/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/zaknesler/modulate/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/zaknesler/modulate/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/zaknesler/modulate/compare/v0.1.3...v0.2.0
[0.1.3]: https://github.com/zaknesler/modulate/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/zaknesler/modulate/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/zaknesler/modulate/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/zaknesler/modulate/releases/tag/v0.1.0
