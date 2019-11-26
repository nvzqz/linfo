//! License info.

#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std as core;

use core::{
    convert::{TryFrom, TryInto},
    fmt,
};

#[macro_use]
mod macros;
mod permissions;
mod requirements;
mod util;

pub mod expr;
pub mod spdx;

#[doc(inline)]
pub use self::{
    expr::Expr,
    permissions::Permissions,
    requirements::Requirements,
    spdx::SpdxLicense,
};

/// A known license.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum License {
    /// A commonly found license listed [here](https://spdx.org/licenses).
    Spdx(SpdxLicense),
    // TODO: Replace with `#[non_exhaustive]` when stable
    #[doc(hidden)]
    _NonExhaustive(util::Never),
}

impl From<SpdxLicense> for License {
    #[inline]
    fn from(l: SpdxLicense) -> Self {
        Self::Spdx(l)
    }
}

impl fmt::Display for License {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Spdx(l) => l.fmt(f),
            Self::_NonExhaustive(never) => never.consume(),
        }
    }
}

impl<'a> TryFrom<&'a str> for License {
    type Error = ParseError<'a>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let s = s.trim();
        // Ok while SPDX is the only variant
        SpdxLicense::parse(s).map(|l| l.into())
    }
}

impl License {
    /// Attempts to parse `input` and returns a [`ParseError`] on error.
    #[inline]
    pub fn parse<'a, I>(input: I) -> Result<Self, ParseError<'a>>
        where I: TryInto<Self, Error = ParseError<'a>>
    {
        input.try_into()
    }

    /// Returns the string identifier of this license. This is usually the same
    /// string used to parse the license.
    #[inline]
    pub fn id(&self) -> &str {
        match self {
            License::Spdx(l) => l.id(),
            License::_NonExhaustive(never) => never.consume(),
        }
    }

    /// Returns the permissions expressed by this license, if they are known.
    #[inline]
    pub fn permissions(&self) -> Option<Permissions> {
        match self {
            License::Spdx(l) => Some(l.permissions()),
            _ => None,
        }
    }

    /// Returns the requirements expressed by this license, if they are known.
    #[inline]
    pub fn requirements(&self) -> Option<Requirements> {
        match self {
            License::Spdx(l) => Some(l.requirements()),
            _ => None,
        }
    }
}

/// An error returned when attempting to parse a [`License`](enum.License.html).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    /// Empty string provided.
    Empty,
    /// An error returned when a license name is unknown.
    UnknownLicenseId(&'a str),
}

impl fmt::Display for ParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Empty => {
                write!(f, "empty string provided")
            },
            ParseError::UnknownLicenseId(id) => {
                write!(f, "'{}' is not a known license ID", id)
            },
        }
    }
}
