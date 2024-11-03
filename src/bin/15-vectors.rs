fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // println!("{}", vec);
    // The above does not work because vec is a struct behind the scenes and we cannot print the entire struct together.
    // Solution: Debug trait(will be discussed later)
    println!("{:?}", vec);

    // an unrelated example to use type using generics is:
    let numbers : Vec<i32> = vec![1,2,3,4,5];
    for i in numbers {
        println!("{}", i);
    }
}
