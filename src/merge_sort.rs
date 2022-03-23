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
    if slice.len() <= 1 {
        return;
    }

    let slice_mid = slice.len() / 2;

    merge_sort(&mut slice[..slice_mid]);
    merge_sort(&mut slice[slice_mid..]);
    merge(slice, slice_mid);
}

fn merge<T>(slice: &mut [T], slice_mid: usize)
where
    T: Ord + Copy,
{
    let first_slice = slice[..slice_mid].to_vec();
    let second_slice = slice[slice_mid..].to_vec();

    let mut first_slice_index = 0;
    let mut second_slice_index = 0;

    for i in 0..slice.len() {
        if first_slice_index < first_slice.len() {
            if second_slice_index < second_slice.len() {
                if first_slice[first_slice_index] <= second_slice[second_slice_index] {
                    slice[i] = first_slice[first_slice_index];
                    first_slice_index += 1;
                } else {
                    slice[i] = second_slice[second_slice_index];
                    second_slice_index += 1;
                }
            } else {
                slice[i] = first_slice[first_slice_index];
                first_slice_index += 1;
            }
        } else {
            slice[i] = second_slice[second_slice_index];
            second_slice_index += 1;
        }
    }
}
