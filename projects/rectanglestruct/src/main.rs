struct Area {
    width: u32,
    height: u32,

}

fn main() {
    let rect = Area {
        width: 30,
        height: 50,
    };

    println!("the area is {}", areall(&rect));
}
fn areall(areal: &Area) -> u32{
    areal.width * areal.height
}