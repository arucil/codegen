use std::fmt::{self, Write};

use crate::formatter::Formatter;


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

    pub fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        for line in self.docs.lines() {
            writeln!(fmt, "/// {}", line)?;
        }

        Ok(())
    }
}
