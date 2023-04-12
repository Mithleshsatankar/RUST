use std::io;

fn find_first_occurrence(arr: &[i32], num: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;

        if arr[mid] == num {
            // Check if this is the first occurrence of num
            if mid == 0 || arr[mid - 1] < num {
                return Some(mid);
            } else {
                right = mid - 1;
            }
        } else if arr[mid] < num {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    None
}

fn main() {
    // Prompt the user to enter a sorted array of integers
    println!("Enter a sorted array of integers, separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    // Prompt the user to enter a number to search for
    println!("Enter a number to search for:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num: i32 = input.trim().parse().unwrap();

    // Call the find_first_occurrence function to find the index of the first occurrence of num
    match find_first_occurrence(&arr, num) {
        Some(index) => println!("The first occurrence of {} is at index {}", num, index),
        None => println!("{} is not in the array", num),
    }
}
