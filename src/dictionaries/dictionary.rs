use crate::{Identifier, TypeError};
use std::collections::HashMap;

#[derive(Clone)]
pub(super) struct Dictionary<T> {
    pub(super) entries: HashMap<Identifier, T>,
}

impl<T> Default for Dictionary<T> {
    fn default() -> Self {
        Dictionary {
            entries: Default::default(),
        }
    }
}

impl<T> Dictionary<T> {
    pub(super) fn get(&self, id: &Identifier) -> crate::Result<&T> {
        self.entries.get(id).ok_or_else(|| TypeError {
            msg: format!("Variable `{}` not found.", id),
        })
    }
    pub(super) fn names(&self) -> impl Iterator<Item = &Identifier> + Clone {
        self.entries.keys()
    }
}
