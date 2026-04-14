---
bump: minor
---

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
