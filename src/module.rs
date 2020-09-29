use std::fmt::{self, Write};

use crate::docs::Docs;
use crate::formatter::Formatter;
use crate::function::Function;
use crate::scope::Scope;

use crate::r#enum::Enum;
use crate::r#impl::Impl;
use crate::r#struct::Struct;
use crate::r#trait::Trait;
use crate::r#type::Type;


/// Defines a module.
#[derive(Debug, Clone)]
pub struct Module {
    /// Module name
    pub name: String,

    /// Visibility
    vis: Option<String>,

    /// Module documentation
    docs: Option<Docs>,

    /// Contents of the module
    scope: Scope,
}


impl Module {
    /// Return a new, blank module
    pub fn new(name: impl Into<String>) -> Self {
        Module {
            name: name.into(),
            vis: None,
            docs: None,
            scope: Scope::new(),
        }
    }

    /// Returns a mutable reference to the module's scope.
    pub fn scope(&mut self) -> &mut Scope {
        &mut self.scope
    }

    /// Set the module visibility.
    pub fn vis(&mut self, vis: impl Into<String>) -> &mut Self {
        self.vis = Some(vis.into());
        self
    }

    /// Import a type into the module's scope.
    ///
    /// This results in a new `use` statement bein added to the beginning of the
    /// module.
    pub fn import(
        &mut self,
        path: impl Into<String>,
        ty: impl AsRef<str>,
    ) -> &mut Self {
        self.scope.import(path, ty);
        self
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
        self.scope.new_module(name)
    }

    /// Returns a reference to a module if it is exists in this scope.
    pub fn get_module<Q: ?Sized>(&self, name: &Q) -> Option<&Module>
    where
        String: PartialEq<Q>,
    {
        self.scope.get_module(name)
    }

    /// Returns a mutable reference to a module if it is exists in this scope.
    pub fn get_module_mut<Q: ?Sized>(&mut self, name: &Q) -> Option<&mut Module>
    where
        String: PartialEq<Q>,
    {
        self.scope.get_module_mut(name)
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
        self.scope.get_or_new_module(name)
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
        self.scope.push_module(item);
        self
    }

    /// Push a new struct definition, returning a mutable reference to it.
    pub fn new_struct(&mut self, name: impl Into<String>) -> &mut Struct {
        self.scope.new_struct(name)
    }

    /// Push a structure definition
    pub fn push_struct(&mut self, item: Struct) -> &mut Self {
        self.scope.push_struct(item);
        self
    }

    /// Push a new function definition, returning a mutable reference to it.
    pub fn new_fn(&mut self, name: impl Into<String>) -> &mut Function {
        self.scope.new_fn(name)
    }

    /// Push a function definition
    pub fn push_fn(&mut self, item: Function) -> &mut Self {
        self.scope.push_fn(item);
        self
    }

    /// Push a new enum definition, returning a mutable reference to it.
    pub fn new_enum(&mut self, name: impl Into<String>) -> &mut Enum {
        self.scope.new_enum(name)
    }

    /// Push an enum definition
    pub fn push_enum(&mut self, item: Enum) -> &mut Self {
        self.scope.push_enum(item);
        self
    }

    /// Push a new `impl` block, returning a mutable reference to it.
    pub fn new_impl(&mut self, target: impl Into<Type>) -> &mut Impl {
        self.scope.new_impl(target)
    }

    /// Push an `impl` block.
    pub fn push_impl(&mut self, item: Impl) -> &mut Self {
        self.scope.push_impl(item);
        self
    }

    /// Push a trait definition
    pub fn push_trait(&mut self, item: Trait) -> &mut Self {
        self.scope.push_trait(item);
        self
    }

    /// Formats the module using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        if let Some(ref vis) = self.vis {
            write!(fmt, "{} ", vis)?;
        }

        write!(fmt, "mod {}", self.name)?;
        fmt.block(|fmt| self.scope.fmt(fmt))
    }
}
