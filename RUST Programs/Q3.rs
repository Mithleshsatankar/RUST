fn find_shortest_word(s: &str) -> &str {
    s.split_whitespace()
        .min_by_key(|word| word.len())
        .unwrap_or("")
}

fn main() {
    let s = "The quick brown fox jumps over the lazy dog";
    let shortest_word = find_shortest_word(s);
    println!("The shortest word in '{}' is '{}'", s, shortest_word);
}