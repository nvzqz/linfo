//! License info.

#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std as core;

#[macro_use]
mod macros;

pub mod spdx;

#[doc(inline)]
pub use self::spdx::SpdxLicense;

/// A known license.
pub enum License {
    /// A commonly found license listed [here](https://spdx.org/licenses).
    Spdx(SpdxLicense),
    // TODO: Replace with `#[non_exhaustive]` when stable
    #[doc(hidden)]
    _NonExhaustive(core::convert::Infallible),
}
