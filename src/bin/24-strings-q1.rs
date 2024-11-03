// Write a function that takes a string as an input and returns the first word from it
// Approach 1
// fn main() {
//     let name = String::from("hello world");
//     let ans = first_word(name);
//     println!("ans is {}", ans);
// }

// fn first_word(str: String) -> String {
//     let mut ans = String::from("");
//     for i in str.chars() {
//         if i == ' ' {
//             break;
//         }

//         ans.push_str(&i.to_string());
//     }
//     return ans;
// }

// Approach 2(with slices)
fn main() {
    let name = String::from("hello world");
    let ans = first_word(&name);
    println!("ans is {}", ans);
}

// Change the parameter type to `&str` and rename `str` to `s`
fn first_word(s: &str) -> &str {
    let mut space_index = 0;
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            space_index = i;
            break;
        }
    }
    &s[0..space_index]
}
