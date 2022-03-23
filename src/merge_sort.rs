//! Merge sort algorithm implementation.

/// Merge sort implementation working on types implementing
/// `Ord` + `Copy` so it's mostly usefull for primitive types.
///
/// Examples:
/// ```
/// use algorithms::merge_sort;
///
/// let mut slice = [1, 8, 2, 3, 9, 5];
///
/// merge_sort::merge_sort(&mut slice);
///
/// assert_eq!([1, 2, 3, 5, 8, 9], slice);
/// ```

pub fn merge_sort<T>(slice: &mut [T])
where
    T: Ord + Copy,
{

}
