#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// A type that is not yet resolved.
    /// This can either be a type that is not yet defined (e.g. a custom class, Some(X)), or a type that is not yet inferred (None).
    Unresolved(Option<String>),

    Any,
    Void,
    Integer,
    String,
}

impl Default for Type {
    fn default() -> Self {
        Self::Unresolved(None)
    }
}
