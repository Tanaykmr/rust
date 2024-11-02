fn create_string() {
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{}", s2);
}
// The above won't work because "Hello" can only have 1 owner.
// So, rust has MOVED the ownership of "Hello" in the stack from s1 to s2.
// S1 cannot be used anymore.
// Another solution can be to `clone` s1 if we do not want to get rid of s1:
// `let s2 = s1.clone()`

fn main() {
    create_string();
}

