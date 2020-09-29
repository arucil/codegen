use std::fmt::{self, Write};

use crate::fields::Fields;
use crate::formatter::{Formatter, Format};
use crate::r#enum::EnumVariant;

use crate::r#type::Type;


/// Defines an enum variant.
#[derive(Debug, Clone)]
pub struct Variant {
    name: String,
    fields: Fields,
}


impl EnumVariant for Variant {
    /// Return a new enum variant with the given name.
    fn new(name: impl Into<String>) -> Self {
        Variant {
            name: name.into(),
            fields: Fields::Empty,
        }
    }
}


impl Variant {
    /// Add a named field to the variant.
    pub fn named<S, T>(&mut self, name: S, ty: T) -> &mut Self
    where
        S: Into<String>,
        T: Into<Type>,
    {
        self.fields.named(name, ty);
        self
    }

    /// Add a tuple field to the variant.
    pub fn tuple(&mut self, ty: impl Into<Type>) -> &mut Self {
        self.fields.tuple(ty);
        self
    }
}

impl Format for Variant {
    /// Formats the variant using the given formatter.
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.name)?;
        self.fields.fmt(fmt)?;
        writeln!(fmt, ",")
    }
}
