use crate::r#type::Type;


/// Defines a struct field.
#[derive(Debug, Clone)]
pub struct Field {
    /// Field name
    pub name: String,

    /// Field type
    pub ty: Type,

    /// Field documentation
    pub documentation: Vec<String>,

    /// Field annotation
    pub annotation: Vec<String>,
}


impl Field {
    /// Return a field definition with the provided name and type
    pub fn new<S, T>(name: S, ty: T) -> Self
        where
            S: Into<String>,
            T: Into<Type>,
    {
        Field {
            name: name.into(),
            ty: ty.into(),
            documentation: vec![],
            annotation: vec![],
        }
    }

    /// Set field's documentation.
    pub fn doc<II, I, S>(
        &mut self, documentation: II,
    ) -> &mut Self
        where
            II: IntoIterator<IntoIter=I, Item=S>,
            I: Iterator<Item=S>,
            S: Into<String>,
    {
        self.documentation = documentation.into_iter()
            .map(|doc| doc.into())
            .collect();
        self
    }

    /// Set field's annotation.
    pub fn annotation<II, I, S>(
        &mut self,
        annotation: II,
    ) -> &mut Self
        where
            II: IntoIterator<IntoIter=I, Item=S>,
            I: Iterator<Item=S>,
            S: Into<String>,
    {
        self.annotation = annotation.into_iter()
            .map(|ann| ann.into())
            .collect();
        self
    }
}
