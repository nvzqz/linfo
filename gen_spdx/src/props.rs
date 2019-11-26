use std::fmt;

include!("../../src/spdx/props.rs");

impl BoolProperties {
    #[inline]
    pub fn rendered(self) -> impl fmt::Display {
        struct Display(BoolProperties);

        impl fmt::Display for Display {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "BoolProperties::new()")?;
                for prop in BoolProperty::all() {
                    if self.0.contains(prop) {
                        write!(f, ".set_{}(true)", prop.snake_case())?;
                    }
                }
                Ok(())
            }
        }

        Display(self)
    }
}

impl BoolProperty {
    fn snake_case(self) -> &'static str {
        match self {
            Self::FsfLibre => "fsf_libre",
            Self::OsiApproved => "osi_approved",
        }
    }
}
