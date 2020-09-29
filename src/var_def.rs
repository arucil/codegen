use std::fmt::{self, Write};
use crate::r#type::Type;
use crate::formatter::{Formatter, Format};

/// Static variable.
#[derive(Debug, Clone)]
pub struct VarDef {
    kind: VarDefKind,
    /// variable name
    pub name: String,
    ty: Type,
    value: String,

    /// visibility
    pub vis: Option<String>,
}


/// Specifies the kind of a variable definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarDefKind {
    /// Static variable definition.
    Static,
    /// Constant definition.
    Const,
}


impl VarDef {
    /// Return a new variable definition.
    pub fn new<S, T>(kind: VarDefKind, name: S, ty: T) -> Self
    where
        S: Into<String>,
        T: Into<Type>,
    {
        Self {
            kind,
            name: name.into(),
            ty: ty.into(),
            value: "".to_owned(),
            vis: None,
        }
    }

    /// Return a new static variable definition.
    pub fn new_static<S, T>(name: S, ty: T) -> Self
    where
        S: Into<String>,
        T: Into<Type>,
    {
        Self::new(VarDefKind::Static, name, ty)
    }

    /// Return a new constant definition.
    pub fn new_const<S, T>(name: S, ty: T) -> Self
    where
        S: Into<String>,
        T: Into<Type>,
    {
        Self::new(VarDefKind::Const, name, ty)
    }

    /// Set the visibility.
    pub fn vis(&mut self, vis: impl Into<String>) -> &mut Self {
        self.vis = Some(vis.into());
        self
    }

    /// Set the value.
    pub fn value(&mut self, value: impl Into<String>) -> &mut Self {
        self.value = value.into();
        self
    }
}


impl Format for VarDef {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
      match self.kind {
          VarDefKind::Static => write!(fmt, "static")?,
          VarDefKind::Const => write!(fmt, "const")?,
      }
      write!(fmt, " {}: ", self.name)?;
      self.ty.fmt(fmt)?;
      write!(fmt, " = {};", self.value)
  }
}