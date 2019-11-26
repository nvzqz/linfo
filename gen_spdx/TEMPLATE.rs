// This file was generated automagically by `gen_spdx`. Please, for the love of
// all things good, do not attempt to edit this file by hand.
//
// See instead:
// - `gen_spdx/TEMPLATE.rs`
// - `gen_spdx/src/main.rs`

use super::{{Map, Info, BoolProperties}};

/// A commonly found license listed [here](https://spdx.org/licenses).
///
/// This list is based on version 3.7 (2019-10-22). Please submit a pull
/// request or issue if you see that this list is out-of-date.
///
/// **SemVer Compatibility:** this license is intended to have the
/// semantics of `#[non_exhaustive]`. This library reserves the right to
/// add, reorganize, or otherwise adjust variants. These changes are
/// allowed between otherwise API-compatible versions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// TODO: Add `#[non_exhaustive]` when stable
#[deny(non_camel_case_types)]
pub enum SpdxLicense {{
    {license_variants}
}}

impl SpdxLicense {{
    // Defined here to make use of macro repetition; the public API is `COUNT`
    // which is declared with the other public items.
    pub(crate) const _COUNT: usize = {count};

    pub(crate) const _INFO: Info = {info};

    // Creates static a hash map if `phf` is enabled, else resorts to a good ol'
    // `match` statement :D
    pub(crate) fn _from_id(id: &str) -> Option<Self> {{
        #[cfg(feature = "phf")]
        {{
            static ID_TO_LICENSE: phf::Map<&str, SpdxLicense> = phf::phf_map! {{
                {id_to_license}
            }};

            ID_TO_LICENSE.get(id).map(|&l| l)
        }}

        #[cfg(not(feature = "phf"))]
        // This wrapping trick allows for reusing `id_to_license`.
        Some(match id {{
            {id_to_license}
            _ => return None,
        }})
    }}
}}
