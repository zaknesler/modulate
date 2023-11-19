# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Tweak styles

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
