fn main() {
    let index = find_first_a(String::from("preey"));
    match index {
        Some(value) => println!("index is: {}", value),
        None => println!("a not found"),
    }
}

fn find_first_a(s: String) -> Option<i32> {
    for (index, char) in s.chars().enumerate() { // we are able to do this because enumerate function returns a tuple of (index, char)
        if char == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}
