// BINARY SEARCH ALGORITHM
// Given a sorted array of n integers and a target value, determine if the target exists
// in the array in logarithmic time using the binary search algorithm. If target exists
// in the array, print the index of it.

fn main() {
    let arr = vec![5, 7, 25, 26, 29, 37, 52];
    let target = 52;

    match binary_search_recursive(&arr, target) {
        Some(index) => println!("The number {} is at index {}", target, index),
        None => println!("{} was not found.", target),
    }
}

#[allow(dead_code)]
fn binary_search_iterative(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    None
}

fn binary_search_recursive(arr: &[i32], target: i32) -> Option<usize> {
    binary_search_recursive_internal(arr, target, 0, arr.len() - 1)
}

fn binary_search_recursive_internal(
    arr: &[i32],
    target: i32,
    left: usize,
    right: usize,
) -> Option<usize> {
    if left > right {
        return None; // Target not found.
    }

    let mid = left + (right - left) / 2;

    if arr[mid] == target {
        return Some(mid); // Target found at index `mid`.
    } else if arr[mid] > target {
        return binary_search_recursive_internal(arr, target, left, mid - 1); // Search the left half.
    } else {
        return binary_search_recursive_internal(arr, target, mid + 1, right); // Search the right half.
    }
}
