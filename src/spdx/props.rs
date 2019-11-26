// This file is intentionally does not rely on others in order to be used in the
// license generator.

type Mask = u8;

/// Compacted boolean properties for a `SpdxLicense`.
#[derive(Clone, Copy)]
pub struct BoolProperties(Mask);

impl BoolProperties {
    #[inline]
    pub const fn new() -> Self {
        Self(0)
    }

    #[inline]
    pub const fn set(self, prop: BoolProperty, yes: bool) -> Self {
        Self(self.0 | ((prop as Mask) * yes as Mask))
    }

    #[inline]
    pub const fn set_osi_approved(self, yes: bool) -> Self {
        self.set(BoolProperty::OsiApproved, yes)
    }

    #[inline]
    pub const fn set_fsf_libre(self, yes: bool) -> Self {
        self.set(BoolProperty::FsfLibre, yes)
    }

    #[inline]
    pub const fn contains(self, p: BoolProperty) -> bool {
        (self.0 & p.bit()) != 0
    }
}

/// An individual property.
#[derive(Clone, Copy)]
pub enum BoolProperty {
    OsiApproved,
    FsfLibre,
}

#[allow(unused)]
impl BoolProperty {
    const LAST: Self = Self::FsfLibre;

    fn all() -> impl Iterator<Item = Self> {
        (0..=(Self::LAST as u8)).map(|v| unsafe { std::mem::transmute(v) })
    }

    #[inline]
    const fn bit(self) -> Mask {
        1 << self as Mask
    }
}
