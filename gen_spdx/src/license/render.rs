use std::fmt;
use super::License;

pub struct Variants<'a>(pub &'a [License]);

impl fmt::Display for Variants<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for license in self.0 {
            write!(
                f,
                "/// {name} (`{id}`).
                {ident},\n",
                name = license.name,
                id = license.id,
                ident = license.ident,
            )?;
        }
        Ok(())
    }
}

pub struct Info<'a>(pub &'a [License]);

impl fmt::Display for Info<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Info {{ \n\
                ids_with_names: {}, \n\
                bool_properties: {}, \n\
            }}",
            IdsWithNames(self.0),
            BoolProperties(self.0),
        )
    }
}

// [(&str, &str)]
pub struct IdsWithNames<'a>(pub &'a [License]);

impl fmt::Display for IdsWithNames<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for license in self.0 {
            // Using escapes internally in case of quotes
            write!(f, "(r#\"{}\"#, r#\"{}\"#),\n", license.id, license.name)?;
        }
        write!(f, "]")
    }
}

// [BoolProperties]
pub struct BoolProperties<'a>(pub &'a [License]);

impl fmt::Display for BoolProperties<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for license in self.0 {
            write!(f, "{},\n", license.bool_properties().rendered())?;
        }
        write!(f, "]")
    }
}

pub struct IdMapping<'a>(pub &'a [License]);

impl fmt::Display for IdMapping<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for license in self.0 {
            write!(
                f,
                "\"{id}\" => SpdxLicense::{ident},\n",
                id = license.id,
                ident = license.ident,
            )?;
        }
        Ok(())
    }
}
