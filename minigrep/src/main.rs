use minigrep::read_file;
use std::env;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: Vec<String>) -> Self {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        Self {
            query: args[1].clone(),
            file_path: args[2].clone(),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args);
    println!("Querying with:{}", config.query);
    let contents = read_file(&config.file_path).expect("Cannot read file because of error, {}");
    println!("With text:\n{contents}");
}
