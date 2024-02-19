// take an orderable array as input, return it as a sorted vector
fn merge_sort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    // if array has 1 or no elements, it's already sorted
    if arr.len() <= 1 {
        return arr.to_vec();
    }

    // split the arry in half
    let mid = arr.len() / 2;
    let left_half = merge_sort(&arr[..mid]);
    let right_half = merge_sort(&arr[mid..]);

    // merge the two halves back together
    merge(&left_half, &right_half)
}

// take the sliced array as input and merge it back together
fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = vec![];

    // compare elements from left and right, add smaller one to result
    let mut i = 0;
    let mut j = 0;
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i].clone());
            i += 1;
        } else {
            result.push(right[j].clone());
            j += 1;
        }
    }

    // add remaining elements from left and right halves to result
    while i < left.len() {
        result.push(left[i].clone());
        i += 1;
    }
    while j < right.len() {
        result.push(right[j].clone());
        j += 1;
    }

    result
}

fn main() {
    // declare the array
    let arr = vec![5, 8, 3, 6, 1, 9, 4, 7, 2];

    // sort the array using the merge sort function
    let sorted_arr = merge_sort(&arr);

    println!("{:?}", sorted_arr);
}
