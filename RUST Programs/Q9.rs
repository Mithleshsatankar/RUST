fn reverse_string(s: &mut String) {
    let len = s.len();
    let mut char_array: Vec<char> = s.chars().collect();
    for i in 0..len / 2 {
        let j = len - i - 1;
        char_array.swap(i, j);
    }
    s.clear();
    s.push_str(&char_array.iter().collect::<String>());
}
fn main() {
    let mut s = String::from("hello world");
    reverse_string(&mut s);
    println!("{}", s); // prints "dlrow olleh"
}
