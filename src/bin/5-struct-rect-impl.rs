struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn perimeter(&self) -> i32 {
        2 * (self.width + self.height)
    }

    fn debug() -> i32 {
        return 1;
    }
}

fn main() {
    let rect1 = Rect {
        width: 20,
        height: 10,
    };
    println!("The area of the rectangle is: {}", rect1.area());
    println!("The perimeter of the rectangle is: {}", rect1.perimeter());
    // println!("{}", rect1.debug());  // this throws an error since debug does not take &self as an argument, though we can use debug not on an instance, but on Rect itself. It's like a static method in JS
    println!("{}", Rect::debug());
}
