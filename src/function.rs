use std::fmt::{self, Write};

use crate::block::Block;
use crate::body::Body;
use crate::bound::Bound;
use crate::docs::Docs;
use crate::field::Field;
use crate::formatter::{fmt_bounds, fmt_generics};
use crate::formatter::Formatter;

use crate::r#type::Type;


/// Defines a function.
#[derive(Debug, Clone)]
pub struct Function {
    /// Name of the function
    name: String,

    /// Function documentation
    docs: Option<Docs>,

    /// A lint attribute used to suppress a warning or error
    allow: Option<String>,

    /// Function visibility
    vis: Option<String>,

    /// Function generics
    generics: Vec<String>,

    /// If the function takes `&self` or `&mut self`
    arg_self: Option<String>,

    /// Function arguments
    args: Vec<Field>,

    /// Return type
    ret: Option<Type>,

    /// Where bounds
    bounds: Vec<Bound>,

    /// Body contents
    pub body: Option<Vec<Body>>,

    /// Function attributes, e.g., `#[no_mangle]`.
    attributes: Vec<String>,

    /// Function `extern` ABI
    extern_abi: Option<String>,

    /// Whether or not this function is `async` or not
    r#async: bool,
}


impl Function {
    /// Return a new function definition.
    pub fn new(name: impl Into<String>) -> Self {
        Function {
            name: name.into(),
            docs: None,
            allow: None,
            vis: None,
            generics: vec![],
            arg_self: None,
            args: vec![],
            ret: None,
            bounds: vec![],
            body: Some(vec![]),
            attributes: vec![],
            extern_abi: None,
            r#async: false,
        }
    }

    /// Set the function documentation.
    pub fn doc(&mut self, docs: impl Into<String>) -> &mut Self {
        self.docs = Some(Docs::new(docs));
        self
    }

    /// Specify lint attribute to supress a warning or error.
    pub fn allow(&mut self, allow: impl Into<String>) -> &mut Self {
        self.allow = Some(allow.into());
        self
    }

    /// Set the function visibility.
    pub fn vis(&mut self, vis: impl Into<String>) -> &mut Self {
        self.vis = Some(vis.into());
        self
    }

    /// Set whether this function is async or not
    pub fn set_async(&mut self, r#async: bool) -> &mut Self {
        self.r#async = r#async;
        self
    }

    /// Add a generic to the function.
    pub fn generic(&mut self, name: impl Into<String>) -> &mut Self {
        self.generics.push(name.into());
        self
    }

    /// Add `self` as a function argument.
    pub fn arg_self(&mut self) -> &mut Self {
        self.arg_self = Some("self".to_string());
        self
    }

    /// Add `&self` as a function argument.
    pub fn arg_ref_self(&mut self) -> &mut Self {
        self.arg_self = Some("&self".to_string());
        self
    }

    /// Add `&mut self` as a function argument.
    pub fn arg_mut_self(&mut self) -> &mut Self {
        self.arg_self = Some("&mut self".to_string());
        self
    }

    /// Add a function argument.
    pub fn arg<S, T>(&mut self, name: S, ty: T) -> &mut Self
    where
        S: Into<String>,
        T: Into<Type>,
    {
        self.args.push(Field {
            name: name.into(),
            ty: ty.into(),
            // While a `Field` is used here, both `documentation`
            // and `annotation` does not make sense for function arguments.
            // Simply use empty strings.
            documentation: vec![],
            annotation: vec![],
        });

        self
    }

    /// Set the function return type.
    pub fn ret<T>(&mut self, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.ret = Some(ty.into());
        self
    }

    /// Add a `where` bound to the function.
    pub fn bound<S, T>(&mut self, name: S, ty: T) -> &mut Self
    where
        S: Into<String>,
        T: Into<Type>,
    {
        self.bounds.push(Bound {
            name: name.into(),
            bound: vec![ty.into()],
        });
        self
    }

    /// Push a line to the function implementation.
    pub fn line<T>(&mut self, line: T) -> &mut Self
    where
        T: ToString,
    {
        self.body
            .get_or_insert(vec![])
            .push(Body::String(line.to_string()));

        self
    }

    /// Add an attribute to the function.
    ///
    /// ```
    /// use codegen::Function;
    ///
    /// let mut func = Function::new("test");
    ///
    /// // add a `#[test]` attribute
    /// func.attr("test");
    /// ```
    pub fn attr(&mut self, attribute: impl Into<String>) -> &mut Self {
        self.attributes.push(attribute.into());
        self
    }

    /// Specify an `extern` ABI for the function.
    /// ```
    /// use codegen::Function;
    ///
    /// let mut extern_func = Function::new("extern_func");
    ///
    /// // use the "C" calling convention
    /// extern_func.extern_abi("C");
    /// ```
    pub fn extern_abi(&mut self, abi: impl Into<String>) -> &mut Self {
        self.extern_abi.replace(abi.into());
        self
    }

    /// Push a block to the function implementation
    pub fn push_block(&mut self, block: Block) -> &mut Self {
        self.body.get_or_insert(vec![]).push(Body::Block(block));

        self
    }

    /// Formats the function using the given formatter.
    pub fn fmt(&self, is_trait: bool, fmt: &mut Formatter) -> fmt::Result {
        if let Some(ref docs) = self.docs {
            docs.fmt(fmt)?;
        }

        if let Some(ref allow) = self.allow {
            write!(fmt, "#[allow({})]\n", allow)?;
        }

        for attr in self.attributes.iter() {
            write!(fmt, "#[{}]\n", attr)?;
        }

        if is_trait {
            assert!(
                self.vis.is_none(),
                "trait fns do not have visibility modifiers"
            );
        }

        if let Some(ref vis) = self.vis {
            write!(fmt, "{} ", vis)?;
        }

        if let Some(ref extern_abi) = self.extern_abi {
            write!(fmt, "extern \"{extern_abi}\" ", extern_abi = extern_abi)?;
        }

        if self.r#async {
            write!(fmt, "async ")?;
        }

        write!(fmt, "fn {}", self.name)?;
        fmt_generics(&self.generics, fmt)?;

        write!(fmt, "(")?;

        if let Some(ref s) = self.arg_self {
            write!(fmt, "{}", s)?;
        }

        for (i, arg) in self.args.iter().enumerate() {
            if i != 0 || self.arg_self.is_some() {
                write!(fmt, ", ")?;
            }

            write!(fmt, "{}: ", arg.name)?;
            arg.ty.fmt(fmt)?;
        }

        write!(fmt, ")")?;

        if let Some(ref ret) = self.ret {
            write!(fmt, " -> ")?;
            ret.fmt(fmt)?;
        }

        fmt_bounds(&self.bounds, fmt)?;

        match self.body {
            Some(ref body) => fmt.block(|fmt| {
                for b in body {
                    b.fmt(fmt)?;
                }

                Ok(())
            }),
            None => {
                if !is_trait {
                    panic!("impl blocks must define fn bodies");
                }

                write!(fmt, ";\n")
            }
        }
    }
}
