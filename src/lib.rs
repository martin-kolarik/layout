#![feature(bool_to_option)]

#[cfg(feature = "color")]
mod color;
#[cfg(feature = "color")]
pub use color::*;

#[cfg(feature = "layout")]
mod element;
#[cfg(feature = "layout")]
pub use element::*;

#[cfg(feature = "layout")]
mod error;
#[cfg(feature = "layout")]
pub use error::*;

#[cfg(feature = "layout")]
mod font;
#[cfg(feature = "layout")]
pub use font::*;

#[cfg(feature = "layout")]
#[macro_use]
mod layout;
#[cfg(feature = "layout")]
pub use self::layout::*;
