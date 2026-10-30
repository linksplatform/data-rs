---
bump: patch
---

### Fixed
- Start the default external reference range one address after `continue`, keeping all service constants out of reference classification.
- Reject custom external ranges that overlap the supplied internal range or its reserved service values in `LinksConstants::full_new` and `LinksConstants::via_ranges`.
