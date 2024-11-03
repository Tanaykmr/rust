fn main() {
    let mut name = String::from("Tanay");
    println!("name is {}", name);
    name.push_str(" Kumar");
    println!("name is {}", name);
    name.replace_range(5..name.len(), "");
    println!("name is {}.", name);
}
