/// Defines an import (`use` statement).
#[derive(Debug, Clone)]
pub struct Import {
    line: String,

    /// Function visibility
    pub vis: Option<String>,
}


impl Import {
    /// Return a new import.
    pub fn new(path: impl AsRef<str>, ty: impl AsRef<str>) -> Self {
        Import {
            line: format!("{}::{}", path.as_ref(), ty.as_ref()),
            vis: None,
        }
    }

    /// Set the import visibility.
    pub fn vis(&mut self, vis: impl Into<String>) -> &mut Self {
        self.vis = Some(vis.into());
        self
    }
}
