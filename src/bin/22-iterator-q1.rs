fn main() {
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter = v1.iter();

    let iter2 = iter.filter(|x| *x % 2 != 0).map(|x| x * 2);
    for x in iter2 {
        println!("{}", x);
    }
}
// try using .collect()