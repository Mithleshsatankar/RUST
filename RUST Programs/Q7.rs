fn kth_smallest_element(array: &[i32], k: usize) -> i32 {
    let mut sorted_array = array.to_vec();
    sorted_array.sort();
    sorted_array[k - 1]
}

fn main() {
    let array = [10, 7, 3, 8, 4, 2, 5];
    let k = 3;
    let kth_smallest = kth_smallest_element(&array, k);
    println!("The {}th smallest element in {:?} is {}", k, array, kth_smallest);
}
