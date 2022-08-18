use crate::LinkType;
use funty::Integral;
use std::ops::{Div, Sub};

#[derive(Debug, Clone, Copy, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub struct Hybrid<T> {
    value: T,
}

impl<T: LinkType> Hybrid<T> {
    pub const fn new(value: T) -> Self {
        Self::internal(value)
    }

    pub const fn half() -> T
    where
        T: ~const Div<Output = T>,
    {
        T::MAX / T::funty(2)
    }

    pub const fn external(value: T) -> Self
    where
        T: ~const Integral + ~const Sub,
    {
        Self {
            value: Self::extend_value(value),
        }
    }

    pub const fn internal(value: T) -> Self {
        Self { value }
    }

    const fn extend_value(value: T) -> T
    where
        T: ~const Integral + ~const Sub,
    {
        (T::MAX - value).wrapping_add(T::funty(1))
    }

    pub const fn is_zero(&self) -> bool
    where
        T: ~const Default + ~const PartialEq,
    {
        self.value == T::funty(0)
    }

    pub const fn is_internal(&self) -> bool
    where
        T: ~const Div + ~const PartialOrd,
    {
        self.value < Self::half() // || self.value == T::default()
    }

    pub const fn is_external(&self) -> bool
    where
        T: ~const Div + ~const PartialOrd + ~const PartialEq,
    {
        !self.is_internal() || self.value == T::funty(0)
    }

    pub const fn abs(&self) -> T
    where
        T: ~const Integral,
    {
        self.value.wrapping_add(T::funty(1)).wrapping_add(T::MAX)
    }

    pub const fn as_inner(&self) -> T {
        self.value
    }
}