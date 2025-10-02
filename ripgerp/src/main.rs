use std::env;        
use std::fs;
use std::process;
use std::error::Error;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filepath);

  if let Err(e) = run(config) {
    println!("Application error: {}", e);
    process::exit(1);
  }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let content = fs::read_to_string(config.filepath)?;

    println!("With content:\n{}", content);
    Ok(())

}

struct Config {
    query: String,
    filepath: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filepath = args[2].clone();
        Ok(Config {query, filepath})
    }
}
