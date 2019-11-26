use core::fmt;

/// License requirements.
///
/// Determines what the licensed work does or does not require.
// Defined as a bit field to reserve space; saves 6 bytes per license.
#[derive(Copy, Clone, Default, Hash, Eq, PartialEq)]
pub struct Requirements(pub(crate) u8);

#[deny(unused)]
enum Field {
    IncludeCopyright,
    IncludeLicense,
    IncludeNotice,
    IncludeOriginal,
    IncludeInstallInstructions,
    StateChanges,
    Rename,
    // Remember to resize `Requirements` if more than 8 fields
}

impl Field {
    #[inline]
    const fn bit(self) -> u8 {
        1 << self as u8
    }
}

impl fmt::Debug for Requirements {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Requirements")
            .field("include_copyright", &self.include_copyright())
            .field("include_license", &self.include_license())
            .field("include_notice", &self.include_notice())
            .field("include_original", &self.include_original())
            .field("include_install_instructions", &self.include_install_instructions())
            .field("state_changes", &self.state_changes())
            .field("rename", &self.rename())
            .finish()
    }
}

impl Requirements {
    #[allow(unused)] // TODO: Use this in setting SPDX matrix
    pub(crate) const fn new(
        include_copyright: bool,
        include_license: bool,
        include_notice: bool,
        include_original: bool,
        include_install_instructions: bool,
        state_changes: bool,
        rename: bool,
    ) -> Self {
        Requirements(
            ((include_copyright as u8) << Field::IncludeCopyright as u8) |
            ((include_license as u8) << Field::IncludeLicense as u8) |
            ((include_notice as u8) << Field::IncludeNotice as u8) |
            ((include_original as u8) << Field::IncludeOriginal as u8) |
            ((include_install_instructions as u8) << Field::IncludeInstallInstructions as u8) |
            ((state_changes as u8) << Field::StateChanges as u8) |
            ((rename as u8) << Field::Rename as u8)
        )
    }

    #[inline]
    const fn contains(self, f: Field) -> bool {
        self.0 & f.bit() != 0
    }

    /// The license has no requirements.
    #[inline]
    pub const fn has_none(self) -> bool {
        self.0 == 0
    }

    /// The original copyright must be retained.
    #[inline]
    pub const fn include_copyright(self) -> bool {
        self.contains(Field::IncludeCopyright)
    }

    /// The full license text must be included in modified software.
    #[inline]
    pub const fn include_license(self) -> bool {
        self.contains(Field::IncludeLicense)
    }

    /// If the library has a `NOTICE` file with attribution notes, the file must
    /// be included when distributing. You may append to the `NOTICE` file.
    #[inline]
    pub const fn include_notice(self) -> bool {
        self.contains(Field::IncludeNotice)
    }

    /// Copies of the original software or instructions to obtain copies must be
    /// distributed with the software.
    #[inline]
    pub const fn include_original(self) -> bool {
        self.contains(Field::IncludeOriginal)
    }

    /// Must include build & install instructions.
    #[inline]
    pub const fn include_install_instructions(self) -> bool {
        self.contains(Field::IncludeInstallInstructions)
    }

    /// Significant changes made to software must be stated.
    #[inline]
    pub const fn state_changes(self) -> bool {
        self.contains(Field::StateChanges)
    }

    /// The original software's name cannot be used if significant changes have
    /// been made.
    #[inline]
    pub const fn rename(self) -> bool {
        self.contains(Field::Rename)
    }
}
