use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3{
        panic!("Insufficient arguments")
    }
    let query = &args[1];
    let filename = &args[2];

    println!("Query: {}", query);
    println!("Filename: {}", filename);
}
