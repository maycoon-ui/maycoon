# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.2](https://github.com/maycoon-ui/maycoon/compare/maycoon-widgets-v0.5.1...maycoon-widgets-v0.5.2) - 2025-11-12

### Added

- Maximum text width
- Switch Widget

### Fixed

- Typos in checkbox.rs

### Other

- `const`-ify possible functions
- Add aggressive inlining
- Use layout::intersects instead of manual hover check
- Document `WidgetId` of Widgets
- Merge branch 'master' of https://github.com/maycoon-ui/maycoon

## [0.5.1](https://github.com/maycoon-ui/maycoon/compare/maycoon-widgets-v0.5.0...maycoon-widgets-v0.5.1) - 2025-11-07

### Fixed

- Unpin issues with Task

### Other

- Make the `WidgetFetcher` use the `Fetcher` as a backend
- Add build configuration for maycoon-widgets

## [0.4.1](https://github.com/maycoon-ui/maycoon/compare/maycoon-widgets-v0.4.0...maycoon-widgets-v0.4.1) - 2025-11-05

### Added

- Unified Vector Graphics Interface
- Reworked task runner architecture

### Fixed

- Unused generic in Task<T>
- Unused future clippy warning

### Other

- Rework task architecture
- Rework Signal architecture
- Rework task architecture
- Update render dependencies
- Re-organize features

## [0.4.0](https://github.com/maycoon-ui/maycoon/compare/maycoon-widgets-v0.3.2...maycoon-widgets-v0.4.0) - 2025-04-29

### Other

- update Cargo.toml dependencies

## [0.3.0](https://github.com/maycoon-ui/maycoon/compare/maycoon-widgets-v0.1.0...maycoon-widgets-v0.3.0) - 2025-01-26

### Other

- Fix clippy lints
- Fix `clippy::doc_markdown` lints
- Update to Vello 0.4
- Add GestureDetector widget and example
- Fix WidgetId typo in image.rs
- Update text.rs
- Update text.rs
- Update text.rs
- Update fetcher.rs docs
- Fix canvas example
- Add WidgetFetcher Widget
- Make self in widget_id immutable
- Add extension traits
- Add canvas widget
- Add `with_value` and `with_on_change` method
- Add `with_image` method
- Add `with_child` method
- Update to Vello 0.3

## [0.1.0](https://github.com/maycoon-ui/maycoon/releases/tag/maycoon-widgets-v0.1.0) - 2024-10-04

### Other

- Add docs
- Add workspace keys
- Add slider widget
- Rename on_changed to on_change
- Add checkbox widget and example
- Fix button.rs docs
- Update text.rs
- Update lib.rs
- Update image.rs
- Update docs
- Fix children invalidation
- Clippy fixes
- New Themes
- Button remake
- Add Val children
- Rename crates and rework state value
