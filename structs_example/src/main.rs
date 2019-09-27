#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //associated function
    fn create_square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 30,
        width: 50,
    };

    let rect2 = Rectangle {
        height: 20,
        width: 30,
    };

    let square1 = Rectangle::create_square(10);

    // println!("rectangle is {:#?}", rectangle);

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    println!("rect 1 can contain rect 2? {}", rect1.can_hold(&rect2));
}
