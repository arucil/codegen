use std::fmt::{self, Write};

use crate::formatter::{Formatter, Format};

use crate::r#enum::EnumVariant;


/// Defines an enum variant.
#[derive(Debug, Clone)]
pub struct DisVariant {
    name: String,
    /// discriminant
    dis: Option<String>,
}


impl EnumVariant for DisVariant {
    /// Return a new enum variant with the given name.
    fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            dis: None,
        }
    }
}


impl DisVariant {
    /// Set the discriminant of the variant.
    pub fn discriminant(&mut self, dis: impl Into<String>) -> &mut Self {
        self.dis = Some(dis.into());
        self
    }
}

impl Format for DisVariant {
    /// Formats the variant using the given formatter.
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.name)?;
        if let Some(dis) = &self.dis {
            write!(fmt, " = {}", dis)?;
        }
        writeln!(fmt, ",")
    }
}