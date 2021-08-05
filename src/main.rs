use std::thread;
use std::time::Duration;

fn main() {
    let hola = vec![1, 2, 3, 4, 7, 6];
    let ordered = merge_sort(hola);
    println!("{:?}", ordered);
}

fn merge_sort(array: Vec<usize>) -> Vec<usize> {
    if array.len() == 1 {
        return array;
    }
    let left = merge_sort(array[..array.len() / 2].to_vec());
    let right = merge_sort(array[array.len() / 2..].to_vec());
    merge(left, right)
}

fn merge(left: Vec<usize>, right: Vec<usize>) -> Vec<usize> {
    let mut merged = Vec::new();
    // Initial indexes of the left ,right and merged subarray
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    // Copy remaining elemens of left, if there are any
    while i < left.len() {
        merged.push(left[i]);
        i += 1;
    }

    // Copy remaining elemens of right, if there are any
    while j < right.len() {
        merged.push(right[j]);
        j += 1;
    }

    merged
}

#[cfg(test)]
mod tests {
    use crate::merge_sort;

    #[test]
    fn it_works() {
        assert_eq!(merge_sort(vec![4, 3]), vec![3, 4]);
    }

    #[test]
    fn test_base_case() {
        assert_eq!(merge_sort(vec![3]), vec![3]);
    }

    #[test]
    fn totally_backwards() {
        assert_eq!(
            merge_sort(vec![100, 90, 50, 14, 9, 7, 3]),
            vec![3, 7, 9, 14, 50, 90, 100]
        );
    }

    #[test]
    fn already_ordered() {
        assert_eq!(merge_sort(vec![3, 4, 5, 6, 7]), vec![3, 4, 5, 6, 7])
    }
}
