use crate::attr_arg::AttrArg;
use crate::formatter::Formatter;
use std::fmt::{self, Write};

/// An inner attribute or outer attribute.
#[derive(Debug, Clone)]
pub struct Attr {
  /// attribute name
  pub name: String,

  /// attribute argument
  pub arg: Option<AttrArg>,
}

impl Attr {
  /// Create an attribute.
  pub fn new(name: impl Into<String>) -> Self {
    Self {
      name: name.into(),
      arg: None,
    }
  }

  /// Set attribute argument.
  pub fn arg(&mut self, arg: AttrArg) -> &mut Self {
    self.arg = Some(arg);
    self
  }

  /// Set attribute argument to a literal expression.
  pub fn arg_expr(&mut self, expr: impl Into<String>) -> &mut Self {
    self.arg = Some(AttrArg::Expr(expr.into()));
    self
  }

  /// Set attribute argument to a delimited token tree.
  pub fn arg_delimited(&mut self, delim: impl Into<String>) -> &mut Self {
    self.arg = Some(AttrArg::Delimited(delim.into()));
    self
  }

  /// Format the attribute.
  pub fn fmt(&self, fmt: &mut Formatter, inner: bool) -> fmt::Result {
    let prefix = if inner {
      "#!"
    } else {
      "#"
    };

    write!(fmt, "{}[{}", prefix, self.name)?;

    match &self.arg {
      Some(AttrArg::Delimited(delim)) => write!(fmt, "({})", delim)?,
      Some(AttrArg::Expr(expr)) => write!(fmt, " = {}", expr)?,
      None => {}
    }

    writeln!(fmt, "]")
  }
}