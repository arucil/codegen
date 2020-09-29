use std::fmt::{self, Write};

use crate::formatter::{Formatter, Format};


#[derive(Debug, Clone)]
pub struct Docs {
    docs: String,
}


impl Docs {
    pub fn new(docs: impl Into<String>) -> Self {
        Docs {
            docs: docs.into(),
        }
    }
}

impl Format for Docs {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        for line in self.docs.lines() {
            writeln!(fmt, "/// {}", line)?;
        }

        Ok(())
    }
}
