fn main() {
    let mut s1 = String::from("Hello");
    s1 = do_something(s1);
    println!("{}", s1);
}

fn do_something(s2: String) -> String {
    println!("{}", s2);
    return s2;
}
// Even here, once the ownership of "hello" transfers to do_something, we're unable to print it. Solution: s1.clone(), or return string
