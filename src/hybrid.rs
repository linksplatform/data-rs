use num_traits::WrappingAdd;
use platform_num::LinkReference;

#[derive(Debug, Clone, Copy, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub struct Hybrid<T> {
    value: T,
}

impl<T: LinkReference + WrappingAdd> Hybrid<T> {
    pub const fn new(value: T) -> Self {
        Self::internal(value)
    }

    #[must_use]
    pub fn half() -> T {
        T::MAX / T::from_byte(2)
    }

    pub fn external(value: T) -> Self {
        Self {
            value: Self::extend_value(value),
        }
    }

    pub const fn internal(value: T) -> Self {
        Self { value }
    }

    fn extend_value(value: T) -> T {
        (T::MAX - value).wrapping_add(&T::from_byte(1))
    }

    pub fn is_zero(&self) -> bool {
        self.value == T::from_byte(0)
    }

    pub fn is_internal(&self) -> bool {
        self.value < Self::half() // || self.value == T::default()
    }

    pub fn is_external(&self) -> bool {
        !self.is_internal() || self.value == T::from_byte(0)
    }

    pub fn abs(&self) -> T {
        self.value
            .wrapping_add(&T::from_byte(1))
            .wrapping_add(&T::MAX)
    }

    pub const fn as_inner(&self) -> T {
        self.value
    }
}
