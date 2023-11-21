# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Updated logo

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
