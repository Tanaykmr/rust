fn main() {
    // consuming adaptors
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("sum is {}", total);
    assert_eq!(total, 6);
    // let sum2: i32 = v1_iter.sum(); // v1_iter cannot be used again
}
