# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/maycoon-ui/maycoon/compare/maycoon-core-v0.2.0...maycoon-core-v0.3.0) - 2025-04-17

### Added

- Plugin System
- Implement widget extensions for components
- hook function to Signal trait
- Remake state management with signals
- Added Mouse Wheel Support
- Add more font configuration options

### Fixed

- IDE Backup files
- Docs for new state changes
- Docs of block_on
- Font Insertion mechanism and documentation
- Replace OS-loaded default font with embedded

### Other

- Replace trace logs with debug logs
- Replace len with capacity for collecting layout
- Update mod.rs
- Update skrifa to 0.30.0
- Update bitflags to 2.9.0
- Update taffy to 0.8.0
- Update vello
- Update winit
- Move ext.rs to maycoon-core
- Remove unused rust features
- Create AppContext
- Added logging
- Fix more `clippy::doc_markdown` lints
- Update bitflags
- Merge branch 'master' of https://github.com/maycoon-ui/maycoon
- Add device selector

## [0.2.0](https://github.com/maycoon-ui/maycoon/compare/maycoon-core-v0.1.0...maycoon-core-v0.2.0) - 2025-01-26

### Other

- Fix typo
- Fix clippy lints
- Fix `clippy::doc_markdown` lints
- Fix updating vello
- Update taffy and winit
- Implement component architecture
- Add size info
- Add Task Runner
- Make self in widget_id immutable
- Add init_threads config parameter
- Update dependencies
- Merge pull request [#28](https://github.com/maycoon-ui/maycoon/pull/28) from waywardmonkeys/update-to-vello-0.3
- Add way to load system fonts
- Replace dashmap with indexmap

## [0.1.0](https://github.com/maycoon-ui/maycoon/releases/tag/maycoon-core-v0.1.0) - 2024-10-04

### Other

- Update config.rs
- Fix non-windows compat
- Add workspace keys
- Add EmptyState
- Rename crates and rework state value
