fn find_median(nums: &[i32]) -> f64 {
    let n = nums.len();
    if n % 2 == 0 {
        (nums[n / 2 - 1] + nums[n / 2]) as f64 / 2.0
    } else {
        nums[n / 2] as f64
    }
}

fn main() {
    let nums = [1, 2, 3, 4, 5, 6];
    let median = find_median(&nums);
    println!("The median of {:?} is {}", nums, median);
}
