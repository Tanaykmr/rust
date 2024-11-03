// Approach 1
// fn main() {
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//     vec.push(4);
//     vec.push(3);
//     vec.push(8);
//     vec.push(7);
//     println!("The old vector is: {:?}", vec);

//     let ans = even_filter1(vec);

//     println!("The resultant vector1 is: {:?}", ans);

// }

// fn even_filter1(vec: Vec<i32>) -> Vec<i32> {
//     println!("The argument vector is: {:?}", vec);
//     let mut new_vec = Vec::new();

//     for val in vec {
//         if val % 2 == 0 {
//             new_vec.push(val);
//         }
//     }

//     return new_vec;
// }

// Approach 2
fn main() {
    let mut vec = vec![1, 2, 3, 4, 3, 8, 7];
    println!("The old vector is: {:?}", vec);

    even_filter2(&mut vec);

    println!("The resultant vector1 is: {:?}", vec);
}

fn even_filter2(vec: &mut Vec<i32>) {
    println!("The argument vector is: {:?}", vec);

    vec.retain(|val: &i32| {
        // The .retain function iterates through all the values of a vector and nly keeps those that satisfy the condition, and removes those that do not
        val % 2 == 0
    });
    println!("Figkltered vector with only even numbers: {:?}", vec);
}
