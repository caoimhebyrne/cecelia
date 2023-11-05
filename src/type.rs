#[derive(Debug, Clone)]
pub enum Type {
    /// A type that is not yet resolved.
    /// This can either be a type that is not yet defined (e.g. a custom class, Some(X)), or a type that is not yet inferred (None).
    Unresolved(Option<String>),

    Void,
    Integer,
    String,
}
