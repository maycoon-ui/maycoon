# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.1](https://github.com/maycoon-ui/maycoon/compare/maycoon-v0.5.0...maycoon-v0.5.1) - 2025-11-07

### Added

- Default impl for NeverTask
- empty function for ListenerRegistry
- Task Fetcher for general usage

### Fixed

- MemoizeSignal not notifying on init
- Docs.rs not compiling maycoon-core

### Other

- Remove Arc and Rc variants of Ref
- Update indexmap to 2.12.0
- Update dependencies
- Make the `WidgetFetcher` use the `Fetcher` as a backend
- Add build configuration for maycoon-widgets

## [0.4.1](https://github.com/maycoon-ui/maycoon/compare/maycoon-v0.4.0...maycoon-v0.4.1) - 2025-11-05

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
- Unused future clippy warning
- Hidden elided lifetime
- Update edition to 2024
- Remove unnecessary Arc's
- Unnecessary  Arc

### Other

- Add tracing-flame example README
- Add github release-plz workflow condition
- Update bitflags to `2.10.0`
- Rework task architecture
- Rework Signal architecture
- Remove parking_lot dependency
- Add cfg aliases
- Remove `VectorGraphicsInterface` lifetime
- Remove system font functionality
- Rework task architecture
- Enable tokio-runner feature for fetcher example
- Remove tokio-runner feature of tracing-flame example
- Optimize imports
- Add tracing instrumentation
- Add tracing-flame example
- Use `tracing` instead of log for logging and tracing
- Update render dependencies
- Update macro dependencies
- Update parking_lot to 0.12.5
- Update font-kit to 0.14.3
- Update bitflags to 2.9.4
- Update taffy to 0.9.1
- Update winit to 0.30.12
- Update nalgebra to 0.34.1
- Update bytemuck to 1.24.0
- Change RenderContext reference to Arc reference
- Update runner() method
- Update context.rs
- Re-organize features

## [0.4.0](https://github.com/maycoon-ui/maycoon/compare/maycoon-v0.3.2...maycoon-v0.4.0) - 2025-04-29

### Added

- Global State Management

### Fixed

- Typo in README

### Other

- Update syn to 2.0.101
- Remove parking_lot dependency
- Update log to 0.4.27
- Update bytemuck to 1.23.0
- Update Cargo.toml
- Update taffy to 0.8.1

## [0.3.0](https://github.com/maycoon-ui/maycoon/compare/maycoon-v0.1.0...maycoon-v0.3.0) - 2025-01-26

### Other

- Fix release-plz
- Configure release-plz
- Fix clippy lints
- Update build.yml
- Update build.yml
- Update README.md
- Update build.yml
- Update build.yml
- Update release.yml
- Update dependencies
- Add canvas widget
- Merge pull request [#28](https://github.com/maycoon-ui/maycoon/pull/28) from waywardmonkeys/update-to-vello-0.3
- Replace dashmap with indexmap
- Update issue templates
- Update README.md

## [0.1.0](https://github.com/maycoon-ui/maycoon/releases/tag/maycoon-v0.1.0) - 2024-10-04

### Fixed

- fixed state issue

### Other

- Update release.yml
- Update README.md
- Create release.yml
- Update build.yml
- Create build.yml
- Add workspace keys
- Update SECURITY.md
- Restyling
- Rename crates and rework state value
- Make may-macro optional
- Add macros feature
- Moved examples to separate workspace
- Format
- Format
- Update rustfmt.toml
- Add READMEs
- Update info
- Fixed merge conflicts
- Reworked state machine
- Update hello-world.rs
- Update counter.rs
- Update dependencies
- Update README.md
- Update hello-world.rs
- Create counter.rs
- Added counter example and macro crate
- Update lib.rs
- Update issue templates
- Update issue templates
- Create CONTRIBUTING.md
- Create SECURITY.md
- Create CODE_OF_CONDUCT.md
- Change Text to Button
- Create rustfmt.toml
- Create clippy.toml
- Hide vello export behind feature gate
- Added Documentation and fixes
- Added Text Rendering
- Update README.md
- Remake (again)
- Remake
- Widgets & Fixes
- Reworked Themes and Styles
- Complete Re-Work ( Again )
- Renderer Revamp
- Added widget lifetimes
- New State Machine
- Update hello-world.rs
- Rename faq.md to FAQ.md
- Fix fmt & theme'ing
- Fix URLs
- Update faq.md
- Update hello-world.rs
- Update Cargo.toml
- Add themes
- Update Cargo.toml
- Update config.toml
- Added themes
- fmt
- Init
