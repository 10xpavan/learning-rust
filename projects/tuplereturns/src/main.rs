fn main() {
    let s1 = String::from("heheboi");

    let (s2, lenght) = takshimoto(s1);

    println!("the length of '{s2}' is {lenght}");
}

fn takshimoto(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}
