use std::collections::HashMap;

fn main(){
    let mut users    = HashMap::new();
    users.insert(String::from("Tanay"), 20);
    users.insert(String::from("raman"), 32);

    let user1 = users.get("Tanay");

    match user1{
        Some(age) => println!("The age is: {}", age),
        None => println!("User not found"),
    }


}
