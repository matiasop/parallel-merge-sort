use rand::{distributions::Uniform, Rng};
use std::sync::mpsc;
use threadpool::ThreadPool;

const N_WORKERS: usize = 4;
const N_JOBS: usize = 4;

fn main() {
    let mut rng = rand::thread_rng();
    let size = 10_000_000;
    let range = Uniform::new(0, size);
    let array: Vec<usize> = (0..size).map(|_| rng.sample(&range)).collect();

    let sorted = parallel_merge_sort(array, N_WORKERS, N_JOBS);
    println!("{:?}", sorted);
}

fn parallel_merge_sort(array: Vec<usize>, n_workers: usize, n_jobs: usize) -> Vec<usize> {
    // n_jobs and n_workers must be a power of 2
    if (n_jobs as f32).log2().fract() != 0.0 || (n_workers as f32).log2().fract() != 0.0 {
        panic!("n_jobs and n_workers must be a power of 2");
    }

    // Check if len of array is larger than n_jobs
    if array.len() < n_jobs {
        panic!("The length of the array should be larger than the number of threads");
    }

    // Create thread pool
    let pool = ThreadPool::new(n_workers);

    let (tx, rx) = mpsc::channel();
    let step = (array.len() as f32 / n_jobs as f32).ceil() as usize;

    for index in 0..n_jobs {
        // Find start and finishing slices for this subarray
        let start = index * step;
        let finish = if (index + 1) * step > array.len() {
            array.len()
        } else {
            (index + 1) * step
        };

        let subarray = array[start..finish].to_owned();
        let tx = tx.clone();

        // Sort the subarray
        pool.execute(move || {
            let sorted_subarray = merge_sort(subarray);
            tx.send(sorted_subarray).unwrap();
        });
    }

    // Recieve and merge the arrays
    let mut subarrays = vec![];
    for _ in 0..n_jobs {
        let recieved = rx.recv().unwrap();
        subarrays.push(recieved);
    }

    // Merge subarrays
    let mut num_of_subarrays = n_jobs;
    while num_of_subarrays > 1 {
        let mut aux = vec![];
        num_of_subarrays /= 2;

        for i in 0..num_of_subarrays {
            aux.push(merge(
                subarrays[i].to_owned(),
                subarrays[subarrays.len() - i - 1].to_owned(),
            ));
        }
        subarrays = aux.to_owned();
    }

    subarrays[0].to_owned()
}

fn merge_sort(array: Vec<usize>) -> Vec<usize> {
    if array.len() == 1 {
        return array;
    }
    let left = merge_sort(array[..array.len() / 2].to_owned());
    let right = merge_sort(array[array.len() / 2..].to_owned());
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

    #[test]
    fn repeated_elements() {
        assert_eq!(merge_sort(vec![3, 3, 3, 2]), vec![2, 3, 3, 3]);
    }
}

#[cfg(test)]
mod parallel_tests {
    use crate::parallel_merge_sort;

    #[test]
    #[should_panic]
    fn wrong_length() {
        let array = vec![1, 3, 4];
        parallel_merge_sort(array, 4, 4);
    }

    #[test]
    #[should_panic]
    fn n_jobs_power_of_2() {
        let array = vec![1, 3, 4, 4, 5, 2, 1, 4, 8];
        parallel_merge_sort(array, 4, 6);
    }

    #[test]
    #[should_panic]
    fn n_workers_power_of_2() {
        let array = vec![1, 3, 4, 4, 5, 2, 1, 4, 8];
        parallel_merge_sort(array, 3, 2);
    }

    #[test]
    fn one_thread() {
        let array = vec![1, 3, 4, 2];
        let sorted = parallel_merge_sort(array, 1, 1);
        let correct = vec![1, 2, 3, 4];
        assert_eq!(sorted, correct);
    }

    #[test]
    fn arrays_if_len_1() {
        let array = vec![1, 3, 4, 2];
        let sorted = parallel_merge_sort(array, 4, 4);
        let correct = vec![1, 2, 3, 4];
        assert_eq!(sorted, correct);
    }
}
