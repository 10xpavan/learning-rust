fn main() {
 gives_ownership();
}

fn gives_ownership() -> String {
    let s1 = String::from("yours");
    println!("{s1}");
}
