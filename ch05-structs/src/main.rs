fn main() {
    let scale = 2;
    let rect = Rectangle{
        width: dbg!(30 * scale),
        height: 50,
    };
    let rect2: Rectangle = Rectangle{
        width: 30,
        height:25,
    };
    let can_hold = rect.can_hold(&rect2);
    println!("The area of the rectable is {} square pixels", rect.area());
    println!("The rect {:?} can hold {:?}: {} ", rect, rect2, can_hold);
    let square = Rectangle::square(3);
    println!("The area of the square {:?} is {}", square, square.area());
    dbg!(&rect);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}