use crate::{Identifier, TypeError};
use std::fmt;

#[derive(Default)]
pub(crate) enum LookupList<'a, T> {
    #[default]
    Empty,
    Extend {
        parent: &'a LookupList<'a, T>,
        var: &'a Identifier,
        val: &'a T,
    },
}

pub struct LookupListIterator<'a, T> {
    ptr: &'a LookupList<'a, T>,
}

impl<T> Clone for LookupListIterator<'_, T> {
    fn clone(&self) -> Self {
        LookupListIterator { ptr: self.ptr }
    }
}

impl<T> Copy for LookupListIterator<'_, T> {}

impl<'a, T> Iterator for LookupListIterator<'a, T> {
    type Item = (&'a Identifier, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        match *self.ptr {
            LookupList::Empty => None,
            LookupList::Extend { parent, var, val } => {
                self.ptr = parent;
                Some((var, val))
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.count();
        (len, Some(len))
    }
}

impl<T> ExactSizeIterator for LookupListIterator<'_, T> {}

impl<'a, T> IntoIterator for &'a LookupList<'a, T> {
    type Item = (&'a Identifier, &'a T);
    type IntoIter = LookupListIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        LookupListIterator { ptr: self }
    }
}

fn lookup_list_fmt_helper<T: fmt::Display>(
    ll: &LookupList<'_, T>,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    match ll {
        LookupList::Empty => Ok(()),
        LookupList::Extend { parent, var, val } => {
            lookup_list_fmt_helper(parent, f)?;
            write!(f, "{} : {}, ", var, val)
        }
    }
}

impl<T: fmt::Display> fmt::Display for LookupList<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LookupList::Empty => write!(f, "\u{00B7}"),
            LookupList::Extend { parent, var, val } => {
                lookup_list_fmt_helper(parent, f)?;
                write!(f, "{} : {}", var, val)
            }
        }
    }
}

impl<'a, T> LookupList<'a, T> {
    pub(super) fn extend_list(&'a self, var: &'a Identifier, val: &'a T) -> LookupList<'a, T> {
        LookupList::Extend {
            parent: self,
            var,
            val,
        }
    }
    pub(super) fn names(&self) -> impl Iterator<Item = &Identifier> + Clone {
        self.into_iter().map(|(var, _)| var)
    }
    pub(super) fn get(&'a self, id: &Identifier) -> crate::Result<&'a T> {
        self.into_iter()
            .find_map(|(var, val)| if var == id { Some(val) } else { None })
            .ok_or_else(|| TypeError {
                msg: format!("Variable `{}` not found.", id),
            })
    }
}
