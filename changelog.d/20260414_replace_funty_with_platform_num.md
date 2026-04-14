---
bump: major
---

### Changed
- Replaced `funty` dependency with `platform-num` (`LinkReference` trait)
- `LinkType` now extends `LinkReference` from `platform-num` instead of `funty::Unsigned`
- Replaced `FuntyPart::funty(n)` calls with `LinkReference::from_byte(n)`
- Added `num-traits` dependency for `WrappingAdd` support

### Removed
- Removed `funty` dependency
- Removed `FuntyPart` trait (replaced by `LinkReference::from_byte`)
- Removed all explicit `TryFrom`/`TryInto` bounds from `LinkType` (now provided by `LinkReference`)
