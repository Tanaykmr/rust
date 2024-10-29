fn main() {
    let x: i32 = 5;
    println!("{}", fib(x));
}

// 0 1 1 2 3 5 8 13

fn fib(num: i32) -> i32 {
    let mut first = 0; // mutable vars are vars whose value you can change. Think of them as non-constants. Every variable you define in rust, by default, is a constant
    let mut second = 1;
    if num == 0 {
        return first;
    }
    if num == 1 {
        return second;
    }

    for _ in 0..num - 1 { // we used _ since we're not using the value of i in the for loop
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;
}
