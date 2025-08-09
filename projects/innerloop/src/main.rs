fn main() {
    let mut count = 0;
    'countingup: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'countingup; //breaks the entier conditions
            }
            remaining -=1;
        }
        count +=1;
    }
    println!("end count = {count}");
}