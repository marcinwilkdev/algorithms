use crate::prelude::*;

pub fn basic_sorting_test<T: Sorter>() {
    let mut slice = [1, 8, 2, 3, 9, 5];
    
    T::sort(&mut slice);
    
    assert_eq!([1, 2, 3, 5, 8, 9], slice);
}

pub fn empty_sorting_test<T: Sorter>() {
    let mut slice = [];
    
    T::sort(&mut slice);

    let expected: [(); 0] = [];
    
    assert_eq!(expected, slice);
}

pub fn sorted_sorting_test<T: Sorter>() {
    let mut slice = [1, 2, 3, 4, 5];
    
    T::sort(&mut slice);
    
    assert_eq!([1, 2, 3, 4, 5], slice);
}

pub fn sorted_backwards_sorting_test<T: Sorter>() {
    let mut slice = [5, 4, 3, 2, 1];
    
    T::sort(&mut slice);
    
    assert_eq!([1, 2, 3, 4, 5], slice);
}
