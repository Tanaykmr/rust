fn main() {
    let mut s1 = String::from("Tanay");
    do_something(&mut s1);
    println!("Finally, s1: {}", s1);
}

fn do_something(s2: &mut String) {
    s2.push_str(" Kumar");
    println!("{}", s2);
}
// The above is how we take a mutable reference