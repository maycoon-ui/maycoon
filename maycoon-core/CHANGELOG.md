# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.3](https://github.com/maycoon-ui/maycoon/compare/maycoon-core-v0.5.2...maycoon-core-v0.5.3) - 2025-11-16

### Added

- Local task running

### Fixed

- Updates always triggering and rework diagnostics

### Other

- Use a ready task for ticking instead of timeout
- Merge branch 'master' of https://github.com/maycoon-ui/maycoon

## [0.5.2](https://github.com/maycoon-ui/maycoon/compare/maycoon-core-v0.5.1...maycoon-core-v0.5.2) - 2025-11-12

### Added

- intersects layout function
- Maximum text width
- Equality layout function
- first_run to Diagnostics

### Fixed

- Broken doctest in layout.rs
- Native task runner functions not configured for native
- Unimplemented LocalTask::take in tokio
- Updates not reaching app until new event

### Other

- Add tests
- `const`-ify possible functions
- Make Listener use rpds::Vector instead of Vec
- Replace indexmap with rpds
- Add #[cold] close function to handler.rs
- Add aggressive inlining
- Use layout::intersects instead of manual hover check
- Make DEFAULT_FONT public
- Remove unnecessary update call
- Make UpdateManager default to Update::Force

## [0.5.1](https://github.com/maycoon-ui/maycoon/compare/maycoon-core-v0.5.0...maycoon-core-v0.5.1) - 2025-11-07

### Added

- Default impl for NeverTask
- empty function for ListenerRegistry
- Task Fetcher for general usage

### Fixed

- MemoizeSignal not notifying on init
- Docs.rs not compiling maycoon-core
- Unpin issues with Task

### Other

- Remove Arc and Rc variants of Ref
- Add build configuration for maycoon-widgets

## [0.4.1](https://github.com/maycoon-ui/maycoon/compare/maycoon-core-v0.4.0...maycoon-core-v0.4.1) - 2025-11-05

### Added

- more tracing calls
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
