#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let react1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {react1:?}")
}