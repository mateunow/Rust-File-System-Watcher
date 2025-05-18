use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let first = &args[1];
    let second = &args[2];
    // dbg!(args);

    
    println!("First argument: {}", first);
    println!("Second argument: {}", second);
} 