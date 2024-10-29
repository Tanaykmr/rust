fn main() {
    println!("{}", is_even(20)); // this is not a function it is a macro. th "{}" tells the compiler to print something dynamic here, and the 2nd argument is what will be printed
}

// i32 is signed, u32 is unsigned
fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}
