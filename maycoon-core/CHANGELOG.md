# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.1](https://github.com/maycoon-ui/maycoon/compare/maycoon-core-v0.4.0...maycoon-core-v0.4.1) - 2025-11-05

### Added

- Signal listeners
- platform module
- futures task runner
- Unified Vector Graphics Interface
- Exit functionality
- Reworked task runner architecture
- Render Context for AppContext
- Diagnostics for AppContext
- ActionSignal signal
- is_locked method for RwSignal
- RwSignal shortcut
- RwSignal based on the RwLock

### Fixed

- Clippy unused variable
- Unused generic in Task<T>
- Hidden elided lifetime
- Remove unnecessary Arc's
- Unnecessary  Arc

### Other

- Add more tracing calls
- Update bitflags to `2.10.0`
- Rework task architecture
- Rework Signal architecture
- Remove parking_lot dependency
- Add cfg aliases
- Remove `VectorGraphicsInterface` lifetime
- Remove system font functionality
- Rework task architecture
- Add tracing instrumentation
- Use `tracing` instead of log for logging and tracing
- Update render dependencies
- Update parking_lot to 0.12.5
- Update font-kit to 0.14.3
- Update bitflags to 2.9.4
- Update taffy to 0.9.1
- Update winit to 0.30.12
- Change RenderContext reference to Arc reference
- Update runner() method
- Update context.rs
- Re-organize features

## [0.4.0](https://github.com/maycoon-ui/maycoon/compare/maycoon-core-v0.3.2...maycoon-core-v0.4.0) - 2025-04-29

### Added

- Global State Management

### Fixed

- Default Font Selection

### Other

- Remove parking_lot dependency
- Update taffy to 0.8.1

## [0.3.1](https://github.com/maycoon-ui/maycoon/compare/maycoon-core-v0.3.0...maycoon-core-v0.3.1) - 2025-04-19

### Other

- Temporarily fix font issues
- Fix cargo asset packaging

## [0.3.0](https://github.com/maycoon-ui/maycoon/compare/maycoon-core-v0.1.0...maycoon-core-v0.3.0) - 2025-01-26

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
