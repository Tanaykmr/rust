struct User {
    first_name: String,
    last_name: String,
    age: i32,
}

fn main() {
    let user = User {
        first_name: String::from("Tanay"),
        last_name: String::from("Kumar"),
        age: 20,
    };

    println!("{}", user.first_name); // we cannot print the entire object since the println macro does not know how to print the entirety of the struct. We will learn how to print the entire struct later
    println!("{}", user.last_name);
}
