fn largest(list: &[i32]) -> &i32 {
    let mut large = &list[0];
    for lag in list {
        if lag > large {
            large = lag;
        }
    }
    large

}

fn main() {
    let num = vec![12, 34, 12, 343, 23, 45, 645, 235, 35, 309];
     
    let result = largest(&num);
    println!("the largest is {}", result);

    let num = vec![234, 5345, 65, 765, 756, 34, 56, 36, 3];
    let result =largest(&num);
    println!("{}", result);
}