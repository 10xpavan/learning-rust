fn main() {
    let rect: (u32, u32) = (30, 50);

    println!("the area is {}", area(rect));
}

fn area(dimensions: (u32, u32)) -> u32{
    dimensions.0*dimensions.1
}
//since this is less meaningfull for the person using this code as they will find it torublesome to know waht exactly is width and height we are going to use structure way of writing this