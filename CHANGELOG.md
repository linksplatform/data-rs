# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- changelog-insert-here -->


## [1.0.0] - 2026-04-14

### Added
- Test coverage infrastructure using cargo-tarpaulin with 90% minimum threshold
- Comprehensive unit tests achieving 94.57% code coverage
- rust-toolchain.toml to pin nightly-2022-08-22 for stable feature compatibility
- Coverage check job in CI/CD pipeline to prevent coverage regression

### Changed
- CI workflow now requires coverage check to pass before build

### Added
- Modern CI/CD pipeline from rust-ai-driven-development-pipeline-template
- GitHub Actions workflow for automated testing, linting, and releases
- Multi-platform testing (Ubuntu, macOS, Windows)
- Pre-commit hooks configuration for code quality
- Fragment-based changelog system
- Automated release workflow with version management
- File size checking for Rust source files
- Contributing guidelines (CONTRIBUTING.md)

### Changed
- Migrated from nightly Rust to **stable Rust** toolchain (requires Rust 1.79+)
- Removed all unstable feature flags:
  - `try_trait_v2` - Flow no longer implements `Try` trait
  - `type_alias_impl_trait` - Point now uses explicit `PointIter` type
  - `const_trait_impl`, `const_convert`, `const_deref`, `const_refs_to_cell`, `const_result_drop` - LinkType/FuntyPart traits are no longer const
  - `step_trait` - LinkType no longer requires `Step` bound
  - `associated_type_bounds` - still used but stabilized in Rust 1.79
- `Flow` type changes:
  - Added `into_control_flow()` method for use with `try_for_each`
  - Removed `Try` and `FromResidual` trait implementations (nightly-only)
- `Point` type changes:
  - Added explicit `PointIter` iterator type (publicly exported)
- `LinkType` trait changes:
  - Removed `Step` trait bound
  - Removed `const` from trait and impl
- `FuntyPart` trait changes:
  - Simplified implementation without const generics
  - Now uses `expect()` instead of `unreachable_unchecked()`
- Updated CI/CD pipeline to use `dtolnay/rust-toolchain@stable`

### Fixed
- Crate now compiles on stable Rust without any feature flags

### Changed
- Migrated all CI/CD scripts from Python to JavaScript ES modules (.mjs) for enhanced performance
- Updated release workflow to use Node.js 20.x for script execution
- Added automatic version bumping based on changelog fragment frontmatter

### Added
- New `get-bump-type.mjs` script that parses changelog fragments and determines version bump type
- Frontmatter support in changelog fragments with `bump: major|minor|patch` specification
- Automatic version bumping during release based on highest priority bump type from fragments

### Documentation
- Updated `changelog.d/README.md` with comprehensive frontmatter documentation and examples
- Updated `CONTRIBUTING.md` with new script references and fragment format instructions

### Changed
- Upgraded to Rust edition 2024 with MSRV 1.85
- Updated all dependencies to latest stable versions (thiserror 2.x, quickcheck 1.1, quickcheck_macros 1.2)
- Migrated CI/CD scripts from Node.js to Rust (rust-script)
- Upgraded CI/CD workflow with change detection, version check, Codecov coverage, and GitHub Pages docs deployment
- Added clippy pedantic and nursery lints configuration
- Added release profile optimization (LTO, single codegen unit, symbol stripping)
- Fixed repository URL and package metadata

### Added
- Converter roundtrip tests and error variant tests (94 total test cases)
- GitHub Pages documentation deployment
- Crates.io publishing support in CI/CD
- Case study document for issue #14

## [0.1.0-beta.3] - Previous Release

### Added

- Core data types and traits for Links Platform
- `LinkType` trait for numeric link identifiers
- `Links` trait for CRUD operations on doublet links storage
- `Flow` type for iteration control (Continue/Break)
- `Query` wrapper with copy-on-write semantics
- `Point` structure for repeating elements
- `Hybrid` type for internal/external link references
- `LinksConstants` for storage configuration
- `AddrToRaw` / `RawToAddr` converters