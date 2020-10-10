use crate::r#type::Type;


/// Defines an associated type.
#[derive(Debug, Clone)]
pub(crate) struct NameTypePair {
    pub(crate) name: String,
    pub(crate) ty: Type,
}
