fn main() {
    let mut arr = [3, 15, 40, 23, 23, 23, 10, 2, 88, 9];
    quicksort(&mut arr);
    println!("Sorted array: {:?}", arr);
}

fn quicksort(arr: &mut [i32]) {
    if arr.len() < 2 {
        return;
    }

    let pivot_index = partition(arr);

    quicksort(&mut arr[..pivot_index]);
    quicksort(&mut arr[pivot_index + 1..])
}

fn partition(arr: &mut [i32]) -> usize {
    let length = arr.len();
    let pivot_index = length / 2;
    arr.swap(pivot_index, length - 1);

    let mut i = 0;
    for j in 0..length - 1 {
        if arr[j] <= arr[length - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, length - 1);
    i
}
