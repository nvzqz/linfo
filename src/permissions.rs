use core::fmt;

/// License permissions.
///
/// Determines what the licensed work does or does not allow.
// Defined as a bit field to reserve space; saves 6 bytes per license.
#[derive(Copy, Clone, Default, Hash, Eq, PartialEq)]
pub struct Permissions(pub(crate) u8);

#[deny(unused)]
enum Field {
    CommercialUse,
    PrivateUse,
    TrademarkUse,
    Distribution,
    Modification,
    PatentRights,
    Sublicense,
    // Remember to resize `Permissions` if more than 8 fields
}

impl Field {
    #[inline]
    const fn bit(self) -> u8 {
        1 << self as u8
    }
}

impl fmt::Debug for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Permissions")
            .field("commercial_use", &self.commercial_use())
            .field("private_use", &self.private_use())
            .field("trademark_use", &self.trademark_use())
            .field("distribution", &self.distribution())
            .field("modification", &self.modification())
            .field("patent_rights", &self.patent_rights())
            .field("sublicense", &self.sublicense())
            .finish()
    }
}

impl Permissions {
    #[allow(unused)] // TODO: Use this in setting SPDX list
    pub(crate) const fn new(
        commercial_use: bool,
        private_use: bool,
        trademark_use: bool,
        distribution: bool,
        modification: bool,
        patent_rights: bool,
        sublicense: bool,
    ) -> Self {
        Permissions(
            ((commercial_use as u8) << Field::CommercialUse as u8) |
            ((private_use as u8) << Field::PatentRights as u8) |
            ((trademark_use as u8) << Field::TrademarkUse as u8) |
            ((distribution as u8) << Field::PrivateUse as u8) |
            ((modification as u8) << Field::Distribution as u8) |
            ((patent_rights as u8) << Field::Modification as u8) |
            ((sublicense as u8) << Field::Sublicense as u8)
        )
    }

    #[inline]
    const fn contains(self, f: Field) -> bool {
        self.0 & f.bit() != 0
    }

    /// The license has no permissions.
    #[inline]
    pub const fn has_none(self) -> bool {
        self.0 == 0
    }

    /// Allows usage for commercial purposes.
    #[inline]
    pub const fn commercial_use(self) -> bool {
        self.contains(Field::CommercialUse)
    }

    /// Allows usage for private purposes.
    #[inline]
    pub const fn private_use(self) -> bool {
        self.contains(Field::PrivateUse)
    }

    /// Allows using contributors' names, trademarks or logos.
    #[inline]
    pub const fn trademark_use(self) -> bool {
        self.contains(Field::TrademarkUse)
    }

    /// Allows for distribution of original or modified (derivative) works.
    #[inline]
    pub const fn distribution(self) -> bool {
        self.contains(Field::Distribution)
    }

    /// Allows for modifications.
    #[inline]
    pub const fn modification(self) -> bool {
        self.contains(Field::Modification)
    }

    /// Provides an express grant of patent rights from contributor.
    #[inline]
    pub const fn patent_rights(self) -> bool {
        self.contains(Field::PatentRights)
    }

    /// Allows for granting/extending to the software.
    #[inline]
    pub const fn sublicense(self) -> bool {
        self.contains(Field::Sublicense)
    }
}
