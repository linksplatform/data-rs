use num_traits::WrappingAdd;
use platform_num::LinkReference;

pub trait LinkType: LinkReference + WrappingAdd {}

impl<T: LinkReference + WrappingAdd> LinkType for T {}
