#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool{
        self.width > 0
    }
}

fn main() {
    let react1 = Rectangle {
        width: 30,
        height: 50,
    };

    if react1.width() {
    println!("the rectanlgel has nonzero width and that is {}", react1.width);
    }
}