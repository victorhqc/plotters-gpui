use std::convert::Infallible;

pub mod backend;
#[cfg(feature = "plotters")]
pub mod element;
mod line;
mod utils;

pub type Error = Infallible;
