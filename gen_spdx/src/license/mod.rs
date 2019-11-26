use regex::{Regex, RegexSet};
use serde::*;
use super::BoolProperties;

pub mod render;

/// A normal license.
pub struct License {
    pub name: String,
    pub ident: String,
    pub id: String,
    pub text: String,
    pub is_osi_approved: bool,
    pub is_fsf_libre: bool,
}

impl<'de> Deserialize<'de> for License {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Parsed::deserialize(d).map(Into::into)
    }
}

impl From<Parsed> for License {
    fn from(l: Parsed) -> Self {
        let ident = l.ident();
        License {
            name: l.name,
            ident,
            id: l.license_id,
            text: l.license_text,
            is_osi_approved: l.is_osi_approved,
            is_fsf_libre: l.is_fsf_libre,
        }
    }
}

impl License {
    pub fn bool_properties(&self) -> BoolProperties {
        BoolProperties::new()
            .set_osi_approved(self.is_osi_approved)
            .set_fsf_libre(self.is_fsf_libre)
    }
}

/// A license parsed from JSON. Acts as a parsing proxy for `License`.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Parsed {
    pub name: String,
    pub license_id: String,
    pub license_text: String,
    #[serde(default)]
    pub is_osi_approved: bool,
    #[serde(default)]
    pub is_fsf_libre: bool,
}

impl Parsed {
    /// Returns a valid Rust `enum` variant identifier for `self.license_id`.
    pub fn ident(&self) -> String {
        match self.license_id.as_str() {
            "0BSD" => "Bsd0".to_owned(),
            id => {
                let mut result = id.to_owned();

                if let Some(start) = result.get_mut(..1) {
                    start.make_ascii_uppercase();
                }

                lazy_static! {
                    static ref CAPS: Regex = Regex::new(
                        r"[A-Z]{2,}"
                    ).unwrap();

                    static ref DASH_LOWER: Regex = Regex::new(
                        r"-[a-z]"
                    ).unwrap();

                    // Replacement: `$1_$2`
                    static ref NUM_DASHES: Regex = Regex::new(
                        r"(\d)-(\d)"
                    ).unwrap();

                    // Replacement: `$1$2`
                    static ref VERSION: Regex = Regex::new(
                        r"(\d)(?:\.0)+([^1-9])*"
                    ).unwrap();

                    // Replacement: `$1$2`
                    static ref CHAR_DASHES1: Regex = Regex::new(
                        r"([a-zA-Z])-([A-Z\d])"
                    ).unwrap();

                    // Replacement: `$1$2`
                    static ref CHAR_DASHES2: Regex = Regex::new(
                        r"(\d)-([A-Z])"
                    ).unwrap();
                }

                for m in CAPS.find_iter(id) {
                    result[(m.start() + 1)..m.end()].make_ascii_lowercase();
                }
                for m in DASH_LOWER.find_iter(id) {
                    result[(m.start() + 1)..m.end()].make_ascii_uppercase();
                }

                CHAR_DASHES1.replace_all(
                    &CHAR_DASHES2.replace_all(
                        &VERSION.replace_all(
                            &NUM_DASHES.replace_all(
                                &result,
                                "${1}_${2}",
                            ),
                            "${1}${2}",
                        ),
                        "${1}${2}",
                    ),
                    "${1}${2}",
                ).replace('.', "_").replace('+', "Plus").replace('-', "")
            },
        }
    }
}
