#![cfg(feature = "serde")]

use core::fmt;
use serde::{
    ser::{Serialize, Serializer},
    de::{self, Deserialize, Deserializer, Visitor},
};
use super::SpdxLicense;

struct LicenseVisitor;

impl<'de> Visitor<'de> for LicenseVisitor {
    type Value = SpdxLicense;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a license string")
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where E: de::Error,
    {
        SpdxLicense::parse(v).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for SpdxLicense {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_str(LicenseVisitor)
    }
}

impl Serialize for SpdxLicense {
    #[inline]
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.id())
    }
}
