fn main() {
    let nums = vec![1, 2, 3, 4];

    // 1
    for value in &nums {
        println!("{}", value);
    }

    println!("-------------");

    // 2
    let iter = nums.iter();
    for value in iter {
        println!("{}", value);
    }

    println!("-------------");
    // 3
    let mut iter2 = nums.iter();
    while let Some(val) = iter2.next() {
        println!("{}", val);
    }

    println!("-------------");
    // 4
    let mut nums2 = vec![1, 2, 3, 4];
    let iter3 = nums2.iter_mut();

    for value in iter3 {
        *value = *value + 1;
    }
    println!("{:?}", nums);

    println!("-------------");
    // 5
    let iter5 = nums.into_iter();

    for value in iter5 {
        println!("{}", value);
    }
}
