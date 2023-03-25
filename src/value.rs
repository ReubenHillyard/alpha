//! Types representing computed values.

pub use crate::dictionaries::Closure;
use crate::Identifier;
use std::fmt;
use std::ops::Deref;

/// The result of a computation.
///
/// A [`Value`] is not a beta-redex at the top level.
#[derive(Clone)]
pub enum Value {
    PiType {
        param_type: Box<Type>,
        tclosure: Closure,
    },
    Lambda {
        closure: Closure,
    },
    Universe,
    Neutral {
        neu: Neutral,
    },
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Value::*;
        match self {
            PiType {
                param_type,
                tclosure,
            } => write!(
                f,
                "\u{220F}({} : {}){}",
                tclosure.param, param_type, tclosure.body
            ),
            Lambda { closure } => write!(f, "\u{03BB}({}){}", closure.param, closure.body),
            Universe => write!(f, "U"),
            Neutral { neu } => write!(f, "{}", neu),
        }
    }
}

/// The record of elimination forms applied to a free variable.
///
/// When a [`Value`] is supplied for the free variable,
/// the elimination forms will be computed, and a [`Value`] obtained.
#[derive(Clone)]
pub enum Neutral {
    Variable(Identifier),
    Application { func: Box<Neutral>, arg: Box<Value> },
}

impl fmt::Display for Neutral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Neutral::*;
        match self {
            Variable(id) => id.fmt(f),
            Application { func, arg } => write!(f, "({})({})", func, arg),
        }
    }
}

/// A [`Value`] which denotes a type.
#[derive(Clone)]
pub struct Type(/*pub(crate)*/ Value);

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for Type {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Type> for Value {
    fn from(value: Type) -> Self {
        value.0
    }
}

impl Type {
    pub(crate) fn create_type_from_value(value: Value) -> Type {
        Type(value)
    }

    /// The universe type.
    pub const UNIVERSE: Type = Type(Value::Universe);
}

pub(crate) struct TypedValue {
    pub type_: Type,
    pub val: Value,
}
