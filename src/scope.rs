extern crate indexmap;


use std::fmt::{self, Write};

use indexmap::IndexMap;

use crate::docs::Docs;
use crate::formatter::{Formatter, Format};
use crate::function::Function;
use crate::import::Import;
use crate::item::Item;
use crate::module::Module;
use crate::dis_variant::DisVariant;
use crate::var_def::VarDef;
use crate::attr::Attr;

use crate::r#enum::Enum;
use crate::r#impl::Impl;
use crate::r#struct::Struct;
use crate::r#trait::Trait;
use crate::r#type::Type;


/// Defines a scope.
///
/// A scope contains modules, types, etc...
#[derive(Debug, Clone)]
pub struct Scope {
    /// Scope documentation
    docs: Option<Docs>,

    /// Inner attributes
    attrs: Vec<Attr>,

    /// Imports
    imports: IndexMap<String, IndexMap<String, Import>>,

    /// Contents of the documentation,
    items: Vec<Item>,
}


impl Scope {
    /// Returns a new scope
    pub fn new() -> Self {
        Scope {
            docs: None,
            imports: IndexMap::new(),
            items: vec![],
            attrs: vec![],
        }
    }

    /// Set the inner documentation.
    pub fn doc(&mut self, docs: impl Into<String>) {
        self.docs = Some(Docs::new(docs));
    }

    /// Push an inner attribute.
    pub fn push_attr(&mut self, attr: Attr) -> &mut Self {
        self.attrs.push(attr);
        self
    }

    /// Create an inner attribute.
    pub fn new_attr(&mut self, name: impl Into<String>) -> &mut Attr {
        self.push_attr(Attr::new(name));

        self.attrs.last_mut().unwrap()
    }

    /// Import a type into the scope.
    ///
    /// This results in a new `use` statement being added to the beginning of
    /// the scope.
    pub fn import(
        &mut self,
        path: impl Into<String>,
        ty: impl AsRef<str>,
    ) -> &mut Import {
        // handle cases where the caller wants to refer to a type namespaced
        // within the containing namespace, like "a::B".
        let path = path.into();
        let ty = ty.as_ref().split("::").next().unwrap_or(ty.as_ref());
        self.imports
            .entry(path.clone())
            .or_insert(IndexMap::new())
            .entry(ty.to_string())
            .or_insert_with(|| Import::new(path, ty))
    }

    /// Push a new module definition, returning a mutable reference to it.
    ///
    /// # Panics
    ///
    /// Since a module's name must uniquely identify it within the scope in
    /// which it is defined, pushing a module whose name is already defined
    /// in this scope will cause this function to panic.
    ///
    /// In many cases, the [`get_or_new_module`] function is preferrable, as it
    /// will return the existing definition instead.
    ///
    /// [`get_or_new_module`]: #method.get_or_new_module
    pub fn new_module(&mut self, name: impl Into<String>) -> &mut Module {
        self.push_module(Module::new(name));

        match self.items.last_mut().unwrap() {
            Item::Module(v) => v,
            _ => unreachable!(),
        }
    }

    /// Returns a mutable reference to a module if it is exists in this scope.
    pub fn get_module_mut<Q: ?Sized>(&mut self, name: &Q) -> Option<&mut Module>
    where
        String: PartialEq<Q>,
    {
        self.items
            .iter_mut()
            .filter_map(|item| match item {
                Item::Module(module) if module.name == *name => Some(module),
                _ => None,
            })
            .next()
    }

    /// Returns a mutable reference to a module if it is exists in this scope.
    pub fn get_module<Q: ?Sized>(&self, name: &Q) -> Option<&Module>
    where
        String: PartialEq<Q>,
    {
        self.items
            .iter()
            .filter_map(|item| match item {
                Item::Module(module) if module.name == *name => Some(module),
                _ => None,
            })
            .next()
    }

    /// Returns a mutable reference to a module, creating it if it does
    /// not exist.
    pub fn get_or_new_module<S>(
        &mut self,
        name: S,
    ) -> &mut Module
        where
            S: AsRef<str> + Into<String>,
    {
        if self.get_module(name.as_ref()).is_some() {
            self.get_module_mut(name.as_ref()).unwrap()
        } else {
            self.new_module(name)
        }
    }

    /// Push a module definition.
    ///
    /// # Panics
    ///
    /// Since a module's name must uniquely identify it within the scope in
    /// which it is defined, pushing a module whose name is already defined
    /// in this scope will cause this function to panic.
    ///
    /// In many cases, the [`get_or_new_module`] function is preferrable, as it will
    /// return the existing definition instead.
    ///
    /// [`get_or_new_module`]: #method.get_or_new_module
    pub fn push_module(&mut self, item: Module) -> &mut Self {
        assert!(self.get_module(&item.name).is_none());
        self.items.push(Item::Module(item));
        self
    }

    /// Push a new struct definition, returning a mutable reference to it.
    pub fn new_struct(&mut self, name: impl Into<String>) -> &mut Struct {
        self.push_struct(Struct::new(name));

        match self.items.last_mut().unwrap() {
            Item::Struct(v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a struct definition
    pub fn push_struct(&mut self, item: Struct) -> &mut Self {
        self.items.push(Item::Struct(item));
        self
    }

    /// Push a new function definition, returning a mutable reference to it.
    pub fn new_fn(&mut self, name: impl Into<String>) -> &mut Function {
        self.push_fn(Function::new(name));

        match self.items.last_mut().unwrap() {
            Item::Function(v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a function definition
    pub fn push_fn(&mut self, item: Function) -> &mut Self {
        self.items.push(Item::Function(item));
        self
    }

    /// Push a new trait definition, returning a mutable reference to it.
    pub fn new_trait(&mut self, name: impl Into<String>) -> &mut Trait {
        self.push_trait(Trait::new(name));

        match self.items.last_mut().unwrap() {
            Item::Trait(v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a trait definition
    pub fn push_trait(&mut self, item: Trait) -> &mut Self {
        self.items.push(Item::Trait(item));
        self
    }

    /// Push a new enum definition, returning a mutable reference to it.
    pub fn new_enum(&mut self, name: impl Into<String>) -> &mut Enum {
        self.push_enum(Enum::new(name));

        match self.items.last_mut().unwrap() {
            Item::Enum(v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a enum definition
    pub fn push_enum(&mut self, item: Enum) -> &mut Self {
        self.items.push(Item::Enum(item));
        self
    }

    /// Push a new enum definition, returning a mutable reference to it.
    pub fn new_discriminant_enum(
        &mut self,
        name: impl Into<String>
    ) -> &mut Enum<DisVariant> {
        self.push_discriminant_enum(Enum::new(name));

        match self.items.last_mut().unwrap() {
            Item::DisEnum(v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a enum definition
    pub fn push_discriminant_enum(
        &mut self,
        item: Enum<DisVariant>,
    ) -> &mut Self {
        self.items.push(Item::DisEnum(item));
        self
    }

    /// Push a new `impl` block, returning a mutable reference to it.
    pub fn new_impl(&mut self, target: impl Into<Type>) -> &mut Impl {
        self.push_impl(Impl::new(target));

        match self.items.last_mut().unwrap() {
            Item::Impl(v) => v,
            _ => unreachable!(),
        }
    }

    /// Push an `impl` block.
    pub fn push_impl(&mut self, item: Impl) -> &mut Self {
        self.items.push(Item::Impl(item));
        self
    }

    /// Push a raw string to the scope.
    ///
    /// This string will be included verbatim in the formatted string.
    pub fn raw(&mut self, val: impl Into<String>) -> &mut Self {
        self.items.push(Item::Raw(val.into()));
        self
    }

    /// Push a new struct definition, returning a mutable reference to it.
    pub fn new_static<S, T>(&mut self, name: S, ty: T) -> &mut VarDef
    where
        S: Into<String>,
        T: Into<Type>,
    {
        self.push_var_def(VarDef::new_static(name, ty));

        match self.items.last_mut().unwrap() {
            Item::VarDef(v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a new struct definition, returning a mutable reference to it.
    pub fn new_const<S, T>(&mut self, name: S, ty: T) -> &mut VarDef
    where
        S: Into<String>,
        T: Into<Type>,
    {
        self.push_var_def(VarDef::new_const(name, ty));

        match self.items.last_mut().unwrap() {
            Item::VarDef(v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a variable definition
    pub fn push_var_def(&mut self, item: VarDef) -> &mut Self {
        self.items.push(Item::VarDef(item));
        self
    }

    /// Return a string representation of the scope.
    pub fn to_string(&self) -> String {
        let mut ret = String::new();

        self.fmt(&mut Formatter::new(&mut ret)).unwrap();

        // Remove the trailing newline
        if let Some(b'\n') = ret.as_bytes().last() {
            ret.pop();
        }

        ret
    }

    fn fmt_imports(&self, fmt: &mut Formatter) -> fmt::Result {
        // First, collect all visibilities
        let mut visibilities = vec![];

        for (_, imports) in &self.imports {
            for (_, import) in imports {
                if !visibilities.contains(&import.vis) {
                    visibilities.push(import.vis.clone());
                }
            }
        }

        let mut tys = vec![];

        // Loop over all visibilities and format the associated imports
        for vis in &visibilities {
            for (path, imports) in &self.imports {
                tys.clear();

                for (ty, import) in imports {
                    if *vis == import.vis {
                        tys.push(ty);
                    }
                }

                if !tys.is_empty() {
                    if let Some(vis) = vis {
                        write!(fmt, "{} ", vis)?;
                    }

                    write!(fmt, "use {}::", path)?;

                    if tys.len() > 1 {
                        write!(fmt, "{{")?;

                        for (i, ty) in tys.iter().enumerate() {
                            if i != 0 {
                                write!(fmt, ", ")?;
                            }
                            write!(fmt, "{}", ty)?;
                        }

                        writeln!(fmt, "}};")?;
                    } else if tys.len() == 1 {
                        writeln!(fmt, "{};", tys[0])?;
                    }
                }
            }
        }

        Ok(())
    }
}


impl Format for Scope {
    /// Formats the scope using the given formatter.
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let has_docs;
        if let Some(docs) = &self.docs {
            has_docs = true;
            docs.fmt(fmt, true)?;
        } else {
            has_docs = false;
        }

        let has_attrs;
        if !self.attrs.is_empty() {
            has_attrs = true;
            if has_docs {
                writeln!(fmt)?;
            }

            for attr in &self.attrs {
                attr.fmt(fmt, true)?;
            }
        } else {
            has_attrs = false;
        }

        let has_imports;
        if !self.imports.is_empty() {
            has_imports = true;
            if has_attrs || has_docs {
                writeln!(fmt)?;
            }
            self.fmt_imports(fmt)?;
        } else {
            has_imports = false;
        }

        let mut newline = has_imports || has_attrs || has_docs;
        for item in &self.items {
            if newline {
                writeln!(fmt)?;
            }
            newline = true;

            match item {
                Item::Module(v) => v.fmt(fmt)?,
                Item::Struct(v) => v.fmt(fmt)?,
                Item::Function(v) => v.fmt(false, fmt)?,
                Item::Trait(v) => v.fmt(fmt)?,
                Item::Enum(v) => v.fmt(fmt)?,
                Item::DisEnum(v) => v.fmt(fmt)?,
                Item::Impl(v) => v.fmt(fmt)?,
                Item::VarDef(v) => v.fmt(fmt)?,
                Item::Raw(v) => {
                    write!(fmt, "{}", v)?;
                }
            }

            writeln!(fmt)?;
        }

        Ok(())
    }
}