fn main() {
    let s1 = String::from("hello world");
    let s2 = firstword(&s1);
    println!("{s2}");
}

fn firstword(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}