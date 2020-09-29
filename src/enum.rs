use std::fmt;

use crate::formatter::{Formatter, Format};
use crate::type_def::TypeDef;
use crate::variant::Variant;

use crate::r#type::Type;


/// Defines an enumeration.
#[derive(Debug, Clone)]
pub struct Enum<V = Variant> {
    type_def: TypeDef,
    variants: Vec<V>,
}

/// A trait
pub trait EnumVariant: Format {
    /// Create a new variant.
    fn new(name: impl Into<String>) -> Self;
}

impl<V: EnumVariant> Enum<V> {
    /// Return a enum definition with the provided name.
    pub fn new(name: impl Into<String>) -> Self {
        Enum {
            type_def: TypeDef::new(name),
            variants: vec![],
        }
    }

    /// Returns a reference to the type.
    pub fn ty(&self) -> &Type {
        &self.type_def.ty
    }

    /// Set the enum visibility.
    pub fn vis(&mut self, vis: impl Into<String>) -> &mut Self {
        self.type_def.vis(vis);
        self
    }

    /// Add a generic to the enum.
    pub fn generic(&mut self, name: impl Into<Type>) -> &mut Self {
        self.type_def.ty.generic(name);
        self
    }

    /// Add a `where` bound to the enum.
    pub fn bound<S, T>(&mut self, name: S, ty: T) -> &mut Self
    where
        S: Into<String>,
        T: Into<Type>,
    {
        self.type_def.bound(name, ty);
        self
    }

    /// Set the enum documentation.
    pub fn doc(&mut self, docs: impl Into<String>) -> &mut Self {
        self.type_def.doc(docs);
        self
    }

    /// Add a new type that the struct should derive.
    pub fn derive(&mut self, name: impl Into<String>) -> &mut Self {
        self.type_def.derive(name);
        self
    }

    /// Specify lint attribute to supress a warning or error.
    pub fn allow(&mut self, allow: impl Into<String>) -> &mut Self {
        self.type_def.allow(allow);
        self
    }

    /// Specify representation.
    pub fn repr(&mut self, repr: impl Into<String>) -> &mut Self {
        self.type_def.repr(repr);
        self
    }

    /// Push a variant to the enum, returning a mutable reference to it.
    pub fn new_variant(&mut self, name: impl Into<String>) -> &mut V {
        self.push_variant(V::new(name));
        self.variants.last_mut().unwrap()
    }

    /// Push a variant to the enum.
    pub fn push_variant(&mut self, item: V) -> &mut Self {
        self.variants.push(item);
        self
    }

}


impl<V: EnumVariant> Format for Enum<V> {
    /// Formats the enum using the given formatter.
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        self.type_def.fmt_head("enum", &[], fmt)?;

        fmt.block(|fmt| {
            for variant in &self.variants {
                variant.fmt(fmt)?;
            }

            Ok(())
        })
    }
}