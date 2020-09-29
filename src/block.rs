use std::fmt::{self, Write};

use crate::body::Body;
use crate::formatter::{Formatter, Format};


/// Defines a code block. This is used to define a function body.
#[derive(Debug, Clone)]
pub struct Block {
    before: Option<String>,
    after: Option<String>,
    body: Vec<Body>,
}


impl Block {
    /// Returns an empty code block.
    pub fn new(before: impl Into<String>) -> Self {
        Block {
            before: Some(before.into()),
            after: None,
            body: vec![],
        }
    }

    /// Push a line to the code block.
    pub fn line<T>(&mut self, line: T) -> &mut Self
    where
        T: ToString,
    {
        self.body.push(Body::String(line.to_string()));
        self
    }

    /// Push a nested block to this block.
    pub fn push_block(&mut self, block: Block) -> &mut Self {
        self.body.push(Body::Block(block));
        self
    }

    /// Add a snippet after the block.
    pub fn after(&mut self, after: impl Into<String>) -> &mut Self {
        self.after = Some(after.into());
        self
    }
}


impl Format for Block {
    /// Formats the block using the given formatter.
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        if let Some(ref before) = self.before {
            write!(fmt, "{}", before)?;
        }

        // Inlined `Formatter::fmt`

        if !fmt.is_start_of_line() {
            write!(fmt, " ")?;
        }

        writeln!(fmt, "{{")?;

        fmt.indent(|fmt| {
            for b in &self.body {
                b.fmt(fmt)?;
            }

            Ok(())
        })?;

        write!(fmt, "}}")?;

        if let Some(after) = &self.after {
            write!(fmt, "{}", after)?;
        }

        writeln!(fmt)
    }
}
