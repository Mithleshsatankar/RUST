fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_so_far = arr[0];
    let mut max_ending_here = arr[0];

    for i in 1..arr.len() {
        max_ending_here = max_ending_here.max(0) + arr[i];
        max_so_far = max_so_far.max(max_ending_here);
    }

    max_so_far
}
fn main() {
    let arr = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let max_sum = max_subarray_sum(&arr);
    println!("Maximum subarray sum: {}", max_sum);
}
