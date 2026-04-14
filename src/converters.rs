use crate::Hybrid;
use num_traits::WrappingAdd;
use platform_num::LinkReference;

#[derive(Default)]
pub struct AddrToRaw;

impl AddrToRaw {
    pub fn convert<T: LinkReference + WrappingAdd>(&self, source: T) -> T {
        Hybrid::external(source).as_inner()
    }
}

#[derive(Default)]
pub struct RawToAddr;

impl RawToAddr {
    pub fn convert<T: LinkReference + WrappingAdd>(&self, source: T) -> T {
        Hybrid::external(source).abs()
    }
}
