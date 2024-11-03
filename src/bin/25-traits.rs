pub trait Summary {
    fn summarize(&self) -> String {
        // this is the default implementation of the summarize function, in case it is not defined explicitly by the implementation.
        return String::from("hi there");
    }
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    // fn summarize(&self) -> String {
    //     return format!("The name                  is {}, and the age is {}", self.name, self.age);
    // }
}

fn main() {
    let user = User {
        name: String::from("Tanay"),
        age: 20,
    };
    println!("{}", user.summarize());
}
