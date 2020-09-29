use std::fmt::{self, Write};

use crate::bound::Bound;
use crate::docs::Docs;
use crate::formatter::{Formatter, Format, fmt_bounds};

use crate::r#type::Type;


/// Defines a type definition.
#[derive(Debug, Clone)]
pub struct TypeDef {
    pub ty: Type,
    vis: Option<String>,
    docs: Option<Docs>,
    derive: Vec<String>,
    allow: Option<String>,
    repr: Option<String>,
    bounds: Vec<Bound>,
    macros: Vec<String>,
}


impl TypeDef {
    /// Return a structure definition with the provided name
    pub fn new(name: impl Into<String>) -> Self {
        TypeDef {
            ty: Type::new(name),
            vis: None,
            docs: None,
            derive: vec![],
            allow: None,
            repr: None,
            bounds: vec![],
            macros: vec![],
        }
    }

    pub fn vis(&mut self, vis: impl Into<String>) {
        self.vis = Some(vis.into());
    }

    pub fn bound<S, T>(&mut self, name: S, ty: T)
    where
        S: Into<String>,
        T: Into<Type>,
    {
        self.bounds.push(Bound {
            name: name.into(),
            bound: vec![ty.into()],
        });
    }

    pub fn r#macro(&mut self, r#macro: impl Into<String>) {
        self.macros.push(r#macro.into());
    }

    pub fn doc(&mut self, docs: impl Into<String>) {
        self.docs = Some(Docs::new(docs));
    }

    pub fn derive(&mut self, name: impl Into<String>) {
        self.derive.push(name.into());
    }

    pub fn allow(&mut self, allow: impl Into<String>) {
        self.allow = Some(allow.into());
    }

    pub fn repr(&mut self, repr: impl Into<String>) {
        self.repr = Some(repr.into());
    }

    pub fn fmt_head(
        &self,
        keyword: impl AsRef<str>,
        parents: &[Type],
        fmt: &mut Formatter
    ) -> fmt::Result {
        if let Some(ref docs) = self.docs {
            docs.fmt(fmt)?;
        }

        self.fmt_allow(fmt)?;
        self.fmt_derive(fmt)?;
        self.fmt_repr(fmt)?;
        self.fmt_macros(fmt)?;

        if let Some(ref vis) = self.vis {
            write!(fmt, "{} ", vis)?;
        }

        write!(fmt, "{} ", keyword.as_ref())?;
        self.ty.fmt(fmt)?;

        if !parents.is_empty() {
            for (i, ty) in parents.iter().enumerate() {
                if i == 0 {
                    write!(fmt, ": ")?;
                } else {
                    write!(fmt, " + ")?;
                }

                ty.fmt(fmt)?;
            }
        }

        fmt_bounds(&self.bounds, fmt)?;

        Ok(())
    }

    fn fmt_allow(&self, fmt: &mut Formatter) -> fmt::Result {
        if let Some(ref allow) = self.allow {
            writeln!(fmt, "#[allow({})]", allow)?;
        }

        Ok(())
    }

    fn fmt_repr(&self, fmt: &mut Formatter) -> fmt::Result {
        if let Some(ref repr) = self.repr {
            writeln!(fmt, "#[repr({})]", repr)?;
        }

        Ok(())
    }

    fn fmt_derive(&self, fmt: &mut Formatter) -> fmt::Result {
        if !self.derive.is_empty() {
            write!(fmt, "#[derive(")?;

            for (i, name) in self.derive.iter().enumerate() {
                if i != 0 {
                    write!(fmt, ", ")?
                }
                write!(fmt, "{}", name)?;
            }

            writeln!(fmt, ")]")?;
        }

        Ok(())
    }

    fn fmt_macros(&self, fmt: &mut Formatter) -> fmt::Result {
        for m in self.macros.iter() {
            writeln!(fmt, "{}", m)?;
        }
        Ok(())
    }
}
