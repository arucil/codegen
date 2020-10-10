
/// The argument of an attribute.
#[derive(Debug, Clone)]
pub enum AttrArg {
  /// delimited token tree, e.g. `#[name(ARG)]`
  Delimited(String),
  /// literal expression, e.g. `#[name = EXPR]`
  Expr(String),
}