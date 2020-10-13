use crate::r#type::Type;


/// Defines a parameter of a function.
#[derive(Debug, Clone)]
pub(crate) struct Param {
    /// modifier of parameter.
    pub(crate) modi: Option<String>,
    pub(crate) name: String,
    pub(crate) ty: Type,
}