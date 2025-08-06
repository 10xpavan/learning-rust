fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = x;
    println!("hey {x}");
}