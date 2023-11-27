# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

- Display playist images
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

[Unreleased]: https://github.com/zaknesler/modulate/compare/v0.7.0...HEAD
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
