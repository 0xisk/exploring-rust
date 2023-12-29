#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Methods
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Assiciated functions
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect1 = Rectangle {
        height: 60,
        width: 40,
    };

    let rect2 = Rectangle {
        height: 2,
        width: 3,
    };

    let rect3 = Rectangle::square(25);

    println!("Rect: {:#?}", rect);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!("Rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("Rect can hold rect2: {}", rect.can_hold(&rect2));
}
