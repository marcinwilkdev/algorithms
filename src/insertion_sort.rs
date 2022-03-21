pub fn insertion_sort<T>(slice: &mut [T])
where
    T: Ord + Copy,
{
    for i in 0..slice.len() {
        let mut j = i;
        let elem = slice[i];

        while j > 0 && slice[j-1] > elem {
            slice[j] = slice[j-1];
            j -= 1
        }

        slice[j] = elem;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertion_sort_works() {
        let mut slice = [1, 8, 2, 3, 9, 5];

        insertion_sort(&mut slice);

        assert_eq!([1, 2, 3, 5, 8, 9], slice);
    }

    #[test]
    fn insertion_sort_zero_size_works() {
        let mut slice: [(); 0] = [];

        insertion_sort(&mut slice);

        let expected: [(); 0] = [];

        assert_eq!(expected, slice);
    }
}
