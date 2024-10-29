fn main(){
    let name = String::from("helloworld");
    let len = get_str_len(name);
    println!("The length of the string is {}", len);
}


fn get_str_len(str: String) -> usize{
    str.chars().count() // this is an implicit return, we do have to write "return" or add a semicolon here
}