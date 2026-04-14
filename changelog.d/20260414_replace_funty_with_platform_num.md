---
bump: major
---

### Changed
- Replaced `funty` dependency with `platform-num` 0.8.0 (`LinkReference` trait)
- Replaced all `LinkType` bounds with `LinkReference` from `platform-num`
- Replaced `FuntyPart::funty(n)` calls with `LinkReference::from_byte(n)`
- Simplified all generic bounds from `T: LinkReference + WrappingAdd` to `T: LinkReference` (`WrappingArithmetic` is now a supertrait of `LinkReference` in `platform-num` 0.8.0)

### Removed
- Removed `funty` dependency
- Removed `num-traits` dependency (re-exported by `platform-num`)
- Removed `LinkType` trait entirely (replaced by `LinkReference`)
- Removed `FuntyPart` trait (replaced by `LinkReference::from_byte`)
- Removed `link_type.rs` module
