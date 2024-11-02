fn main() {
    let s1 = String::from("Hello");
    let s2 = &s1;
    println!("{}", s1);
    println!("{}", s2);
    // This is valid code because s2 is just borrowing the ownership from s1 rather than taking it, and hence both of them can be used to print the string
}
