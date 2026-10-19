/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
use std::cmp::PartialOrd;

fn sort<T: PartialOrd + Clone>(array: &mut [T]){
    if array.len() <= 1 { return; }
    let pivot = 0;
    let len = array.len();
    let mut i = 1;
    let mut j = len - 1;
    loop {
        while i <= j && array[i] < array[pivot] {
            i += 1;
        }
        while i <= j && array[j] > array[pivot] {
            j -= 1;
        }
        if i >= j {
            break;
        }
        array.swap(i, j);
        i += 1;
        j -= 1;
    }
    array.swap(pivot, j);
    sort(&mut array[0..j]);
    sort(&mut array[(j + 1)..len]);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
