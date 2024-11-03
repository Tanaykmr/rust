// Iterator adaptors are methods defined on the Iterator trait that don’t consume the
// iterator. Instead, they produce diﬀerent iterators by changing some aspect of the original iterator.

// fn main() {
//     let v1: Vec<i32> = vec![1, 2, 3];
//     let iter = v1.iter();
//     let iter2 = iter.map(|x| x + 1); // the |x| is the argument, and x+1 is what we're returning

//     for x in iter2 {
//         println!("{}", x);
//     }
//     println!("{:?}", v1);
// }


fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let iter = v1.iter();
    let iter3 = iter.filter(|x| *x % 2 == 0);

    for value in iter3 {
        println!("{}", value);
    }
}

