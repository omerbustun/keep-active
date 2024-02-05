# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2024-02-03
### Added
- 'simulate_activity' function.

### Changed
- Updated dependencies.
- CLI now has a --status-active flag to simulate activity.

## [0.1.0] - 2024-01-18
### Note
- This marks the initial release of the forked version, resetting versioning from the original project. Starting point for the new forked project's versioning system.

---

## Forked from [keepawake-rs](https://github.com/segevfiner/keepawake-rs)
Below is the changelog history from the original project before it was forked.

## Unreleased
### Changed
- **BREAKING** Switched to `derive_builder` for the builder (The builder is now created using `default()`).
- **BREAKING** Renamed `AwakeHandle` to `KeepAwake`.
- Made `winresource` to only be included in the `bin` feature.
- Updated dependencies.

## 0.4.5 - 2023-11-08
### Changed
- Migrate to winresource from winres.
- Make manifest conditional on "bin" feature to avoid it propogating to dependents
  (https://github.com/BenjaminRi/winresource/issues/16).
- Make shadow-rs conditional on "bin" feature.

## 0.4.4 - 2023-10-25
### Changed
- Updated dependencies.

## 0.4.3 - 2023-06-09
### Fixed
- Bad drop for idle display assertion in macOS.

## 0.4.2 - 2023-04-12
### Changed
- Updated dependencies.
- Made `shadow-rs` only be included in the `bin` feature.

## 0.4.1 - 2023-02-26
### Changed
- Updated dependencies.

## 0.4.0 - 2022-12-02
### Changed
- The binary of the crate is now behind the feature `bin` so users of the library crate bring in
  less dependencies.

## 0.3.0 - 2022-11-30
### Changed
- Bump required `zbus`, and add `assume_defaults` to avoid a warning. [#8](https://github.com/segevfiner/keepawake-rs/pull/8)
- Use `impl Into<String>` in builder parameters to make usage more ergonomic. [#9](https://github.com/segevfiner/keepawake-rs/pull/9)

## 0.2.0 - 2022-11-12
### Changed
- The library crate now allows setting the reason and application name used on some operating
  systems. [#5](https://github.com/segevfiner/keepawake-rs/pull/5)
- The libray crate now exposes a builder API instead of a struct.

### Fixed
- Fixed drop panic on Linux when failing to connect to dbus `ScreenSaver`.

## 0.1.3 - 2022-10-14
### Added
- shadow-rs long_version.
- Windows version resource & application manifest.

## 0.1.2 - 2022-10-09
### Added
- Prebuilt binaries.

## 0.1.1 - 2022-10-06
### Changed
- Improve CLI parsing in edge cases, and completions.

## 0.1.0 - 2022-10-02
Initial Release
