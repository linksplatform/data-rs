#![feature(default_free_fn)]
#![feature(associated_type_bounds)]
#![feature(try_trait_v2)]
#![feature(type_alias_impl_trait)]
#![feature(const_trait_impl)]
#![feature(const_convert)]
#![feature(const_result_drop)]

mod constants;
mod converters;
mod flow;
mod hybrid;
mod link_type;
mod links;
mod point;
mod query;

pub use constants::LinksConstants;
pub use converters::{AddrToRaw, RawToAddr};
pub use flow::Flow;
pub use hybrid::Hybrid;
pub use link_type::LinkType;
pub use links::{Links, ReadHandler, WriteHandler};
pub use point::Point;
pub use query::{Query, ToQuery};
