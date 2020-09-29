use std::fmt::{self, Write};

use crate::block::Block;
use crate::formatter::{Formatter, Format};


#[derive(Debug, Clone)]
pub enum Body {
    String(String),
    Block(Block),
}


impl Format for Body {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Body::String(s) => writeln!(fmt, "{}", s),
            Body::Block(b) => b.fmt(fmt),
        }
    }
}
