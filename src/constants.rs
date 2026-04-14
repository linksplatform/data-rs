use std::ops::RangeInclusive;

use crate::Hybrid;
use num_traits::WrappingAdd;
use platform_num::LinkReference;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct LinksConstants<T: LinkReference + WrappingAdd> {
    pub index_part: T,
    pub source_part: T,
    pub target_part: T,
    pub null: T,
    pub r#continue: T,
    pub r#break: T,
    pub skip: T,
    pub any: T,
    pub itself: T,
    pub error: T,
    pub internal_range: RangeInclusive<T>,
    pub external_range: Option<RangeInclusive<T>>,
}

impl<T: LinkReference + WrappingAdd> LinksConstants<T> {
    fn default_target_part() -> T {
        T::from_byte(2)
    }

    pub fn full_new(
        target_part: T,
        internal: RangeInclusive<T>,
        external: Option<RangeInclusive<T>>,
    ) -> Self {
        Self {
            index_part: T::from_byte(0),
            source_part: T::from_byte(1),
            target_part,
            null: T::from_byte(0),
            r#continue: *internal.end(),
            r#break: *internal.end() - T::from_byte(1),
            skip: *internal.end() - T::from_byte(2),
            any: *internal.end() - T::from_byte(3),
            itself: *internal.end() - T::from_byte(4),
            error: *internal.end() - T::from_byte(5),
            internal_range: *internal.start()..=*internal.end() - T::from_byte(6),
            external_range: external,
        }
    }

    // TODO: enough for now
    pub fn via_external(target_part: T, external: bool) -> Self {
        Self::full_new(
            target_part,
            Self::default_internal(external),
            Self::default_external(external),
        )
    }

    pub fn via_ranges(internal: RangeInclusive<T>, external: Option<RangeInclusive<T>>) -> Self {
        Self::full_new(Self::default_target_part(), internal, external)
    }

    #[must_use]
    pub fn via_only_external(external: bool) -> Self {
        Self::via_external(Self::default_target_part(), external)
    }

    #[must_use]
    pub fn external() -> Self {
        Self::via_only_external(true)
    }

    #[must_use]
    pub fn internal() -> Self {
        Self::via_only_external(false)
    }

    #[must_use]
    pub fn new() -> Self {
        Self::internal()
    }

    fn default_internal(external: bool) -> RangeInclusive<T> {
        if external {
            T::from_byte(1)..=Hybrid::half()
        } else {
            T::from_byte(1)..=T::MAX
        }
    }

    fn default_external(external: bool) -> Option<RangeInclusive<T>> {
        if external {
            Some(Hybrid::half()..=T::MAX)
        } else {
            None
        }
    }

    pub fn is_internal(&self, address: T) -> bool {
        self.internal_range.contains(&address)
    }

    pub fn is_external(&self, address: T) -> bool {
        self.external_range
            .clone()
            .is_some_and(|range| range.contains(&address))
    }

    pub fn is_reference(&self, address: T) -> bool {
        self.is_internal(address) || self.is_external(address)
    }
}

impl<T: LinkReference + WrappingAdd> Default for LinksConstants<T> {
    fn default() -> Self {
        Self::new()
    }
}
