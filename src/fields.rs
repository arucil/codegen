use std::fmt::{self, Write};

use crate::field::Field;
use crate::formatter::{Formatter, Format};

use crate::r#type::Type;


/// Defines a set of fields.
#[derive(Debug, Clone)]
pub enum Fields {
    Empty,
    Tuple(Vec<Type>),
    Named(Vec<Field>),
}


impl Fields {
    pub fn push_named(&mut self, field: Field) -> &mut Self
    {
        match self {
            Fields::Empty => {
                *self = Fields::Named(vec![field]);
            }
            Fields::Named(fields) => {
                fields.push(field);
            }
            _ => panic!("field list is named"),
        }

        self
    }

    pub fn named<T>(&mut self, name: impl Into<String>, ty: T) -> &mut Self
    where T: Into<Type>,
    {
        self.push_named(Field::new(name, ty))
    }

    pub fn tuple<T>(&mut self, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        match self {
            Fields::Empty => {
                *self = Fields::Tuple(vec![ty.into()]);
            }
            Fields::Tuple(fields) => {
                fields.push(ty.into());
            }
            _ => panic!("field list is tuple"),
        }

        self
    }
}


impl Format for Fields {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Fields::Named(fields) => {
                fmt.block(|fmt| {
                    for f in fields {
                        for doc in &f.documentation {
                            writeln!(fmt, "/// {}", doc)?;
                        }
                        for ann in &f.annotation {
                            writeln!(fmt, "{}", ann)?;
                        }
                        if let Some(vis) = &f.vis {
                            write!(fmt, "{} ", vis)?;
                        }
                        write!(fmt, "{}: ", f.name)?;
                        f.ty.fmt(fmt)?;
                        writeln!(fmt, ",")?;
                    }

                    Ok(())
                })?;
            }
            Fields::Tuple(tys) => {
                write!(fmt, "(")?;

                let mut comma = false;
                for ty in tys {
                    if comma {
                        write!(fmt, ", ")?;
                    }
                    comma = true;
                    ty.fmt(fmt)?;
                }

                write!(fmt, ")")?;
            }
            Fields::Empty => {}
        }

        Ok(())
    }
}