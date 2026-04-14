---
bump: major
---

### Changed
- Replaced `funty` dependency with `platform-num` (`LinkReference` trait)
- Replaced all `LinkType` bounds with `LinkReference` from `platform-num`
- Replaced `FuntyPart::funty(n)` calls with `LinkReference::from_byte(n)`

### Removed
- Removed `funty` dependency
- Removed `LinkType` trait entirely (replaced by `LinkReference`)
- Removed `FuntyPart` trait (replaced by `LinkReference::from_byte`)
- Removed `link_type.rs` module
