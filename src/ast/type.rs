#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Unresolved(Option<String>),
    None,

    String,
    Integer,
}
