use crate::environment::{Context, Definitions};
use crate::min_excluded;
use crate::value::Closure;
use std::fmt;

/// The name of a variable.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: usize,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a{}", self.name)
    }
}

pub(crate) fn fresh_identifier(defs: &Definitions, ctx: &Context, closure: &Closure) -> Identifier {
    Identifier {
        name: min_excluded::min_excluded(
            defs.names()
                .chain(ctx.names())
                .chain(closure.names())
                .map(|id| id.name),
        ),
    }
}
