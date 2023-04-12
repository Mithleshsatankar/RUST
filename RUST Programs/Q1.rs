fn is_palindrome(input_str: &str) -> bool {
    let reversed_str = input_str.chars().rev().collect::<String>();
    input_str == reversed_str
}
fn main() {
    let input_str = "racecar";
    if is_palindrome(input_str) {
        println!("{} is a palindrome!", input_str);
    } else {
        println!("{} is not a palindrome!", input_str);
    }
}
