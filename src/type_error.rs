/// The error type for type synthesis and type checking.
#[derive(Debug)]
pub struct TypeError {
    pub msg: String,
}

/// A specialised result type for type synthesis and type checking.
///
/// This type is used across [`alpha`](crate) for operations which may produce an error.
///
/// This type alias is provided to avoid clutter from repeatedly writing out [`TypeError`].
///
/// # Examples
///
/// A function that checks whether two identifiers are equal.
///
/// ```
///use alpha::{Identifier, TypeError};
///
/// fn check_equal(a: &Identifier, b: &Identifier) -> alpha::Result<()> {
///     if a == b {
///         Ok(())
///     } else {
///         Err(TypeError {
///             msg: format!("`{}` and `{}` are not equal.", a, b),
///         })
///     }
/// }
///
/// let x = Identifier { name: 0 };
/// let y = Identifier { name: 1 };
/// let z = Identifier { name: 1 };
///
/// assert!(check_equal(&x, &x).is_ok());
/// assert!(check_equal(&x, &y).is_err());
/// assert!(check_equal(&y, &z).is_ok());
/// ```
pub type Result<T> = std::result::Result<T, TypeError>;
