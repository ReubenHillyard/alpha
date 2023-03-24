#![crate_name = "alpha"]

//! Implements type-checking, type-inference and evaluation
//! for expressions in a spartan dependent type theory.

mod dictionaries;
pub mod equivalence;
pub mod evaluation;
pub mod expression;
mod identifier;
mod lists;
pub mod min_excluded;
pub mod read_back;
mod type_error;
pub mod typing;
pub mod value;

pub use crate::identifier::Identifier;
pub use crate::type_error::*;

/// Types and functions associating information to variables.
pub mod environment {
    pub use crate::dictionaries::evaluate_var;
    pub use crate::dictionaries::{Definitions, Environment};
    pub use crate::lists::type_var;
    pub use crate::lists::Context;
}
