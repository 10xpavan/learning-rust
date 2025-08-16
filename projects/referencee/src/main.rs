fn main() {
    let s1 = String::from("hello");

    let len = calculate(&s1);
    println!("the lenght of {s1} is {len}");
}
fn calculate(s: &String) -> usize{        //here s is reference to a string
    s.len()
}   // here s goes out of scope since s does not have ownership, the string wont be droped.