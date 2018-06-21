#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle{width: 10, height: 2};
    println!("rect is {:#?}", rect);
}
