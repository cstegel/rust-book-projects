#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle{width: size, height: size}
    }
}

fn main() {
    let rect1 = Rectangle{width: 1, height: 1};
    let rect2 = Rectangle{width: 2, height: 2};

    println!("rect1 fits in rect2?: {}", rect2.can_hold(&rect1));
    println!("rect2 fits in rect1?: {}", rect1.can_hold(&rect2));

    let square = Rectangle::square(4);
    println!("square: {:#?}", square);
}
