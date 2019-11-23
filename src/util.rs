use core::hash;

// An uninhabitable type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Never(core::convert::Infallible);

impl hash::Hash for Never {
    #[inline]
    fn hash<H: hash::Hasher>(&self, _: &mut H) {
        self.consume()
    }
}

impl Never {
    /// Explicitly tells the compiler that this is unreachable.
    pub fn consume(self) -> ! {
        match self.0 {}
    }
}

#[cfg(feature = "serde")]
mod serde {
    use core::fmt;
    use serde::{
        ser::{Serialize, Serializer},
        de::{Deserialize, Deserializer, Visitor},
    };
    use super::Never;

    struct NeverVisitor;

    impl<'de> Visitor<'de> for NeverVisitor {
        type Value = Never;

        #[inline]
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("uninhabitable")
        }
    }

    impl<'de> Deserialize<'de> for Never {
        #[inline]
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>
        {
            deserializer.deserialize_str(NeverVisitor)
        }
    }

    impl Serialize for Never {
        #[inline]
        fn serialize<S: Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
            self.consume()
        }
    }
}
