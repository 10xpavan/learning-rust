use std::env;         //this is used to interact with the engivornment
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();         //this thing collects the arguments over the command line and sends it to store in the verctorized string format
    
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In the file {file_path}");
    
    
    println!("in file heheboi");
    let contents = fs::read_to_string("B:\\notes\\rust-world\\ripgerp\\heheboi.txt")
         .expect("should have been able to read the file");

    println!("with text:\n{contents}");
}