fn longest_common_prefix<'a>(strings: &'a [&'a str]) -> &'a str {
    if strings.is_empty() {
        return "";
    }

    let shortest_string = strings.iter().min_by_key(|s| s.len()).unwrap();
    let mut prefix = *shortest_string;

    for s in strings {
        while !s.starts_with(prefix) {
            prefix = &prefix[..prefix.len() - 1];
            if prefix.is_empty() {
                return "";
            }
        }
    }

    prefix
}


fn main() {
    let strings = ["flower", "flow", "flight"];
    let prefix = longest_common_prefix(&strings);
    println!("The longest common prefix of {:?} is '{}'", strings, prefix);
}
