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
