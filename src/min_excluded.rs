//! A function to determine the minimum excluded value from an iterator of `usize`.

/// Returns the least `usize` not yielded by its argument.
///
/// # Examples
///
/// ```
/// use alpha::min_excluded::min_excluded;
///
/// assert_eq!(min_excluded([]), 0);
/// assert_eq!(min_excluded([0]), 1);
/// assert_eq!(min_excluded([4]), 0);
/// assert_eq!(min_excluded([2, 0]), 1);
/// assert_eq!(min_excluded([0, 1, 2]), 3);
/// assert_eq!(min_excluded([5, 2, 7]), 0);
/// ```
///
/// # Complexity
///
/// `O(n)` time and space, where `n = iter.into_iter().count()`;
/// so should only be used for small iterables.
///
/// `iter` is cloned, and original and copy are iterated completely.
pub fn min_excluded<I: IntoIterator<Item = usize>>(iter: I) -> usize
where
    I::IntoIter: Clone,
{
    let it = iter.into_iter();
    let len = it.clone().count();
    // minimum excluded value can be at most the number of values yielded by `iter`
    let mut seen = vec![false; len];
    for x in it {
        if x < len {
            seen[x] = true;
        }
    }
    seen.iter().position(|&b| !b).unwrap_or(len)
}
