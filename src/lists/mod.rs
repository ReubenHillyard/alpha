use crate::environment::Definitions;
use crate::lists::list::{LookupList, LookupListIterator};
use crate::value::Type;
use crate::Identifier;

mod list;

/// A typing context.
pub struct Context<'a>(LookupList<'a, Type>);

impl<'a> Context<'a> {
    /// The empty context.
    pub const EMPTY: Context<'a> = Context(LookupList::Empty);

    /// The context extended with a new variable.
    pub fn extend(&'a self, var: &'a Identifier, val: &'a Type) -> Context<'a> {
        Context(self.0.extend_list(var, val))
    }

    /// An iterator over the names of variables in the context.
    pub fn names(&self) -> impl Iterator<Item = &Identifier> + Clone {
        self.0.names()
    }

    fn lookup_type(&self, id: &Identifier) -> crate::Result<&Type> {
        self.0.get(id)
    }

    /// Checks whether a variable is in the context.
    pub fn contains(&self, id: &Identifier) -> crate::Result<()> {
        self.0.get(id).map(|_| ())
    }
}

impl<'a> IntoIterator for &'a Context<'a> {
    type Item = (&'a Identifier, &'a Type);
    type IntoIter = LookupListIterator<'a, Type>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub(crate) type Names<'a> = LookupList<'a, ()>;

impl Names<'_> {
    pub(crate) fn index_of<'a>(&self, id: &'a Identifier) -> Result<usize, &'a Identifier> {
        self.into_iter()
            .enumerate()
            .find_map(|(index, (var, _))| if var == id { Some(index) } else { None })
            .ok_or(id)
    }
    pub(crate) fn extend_names<'a>(&'a self, var: &'a Identifier) -> LookupList<'a, ()> {
        LookupList::Extend {
            parent: self,
            var,
            val: &(),
        }
    }
}

/// Determines the [`Type`] of a variable.
pub fn type_var<'a>(
    defs: &'a Definitions,
    ctx: &'a Context,
    var: &Identifier,
) -> crate::Result<&'a Type> {
    ctx.lookup_type(var).or_else(|_| defs.lookup_type(var))
}
