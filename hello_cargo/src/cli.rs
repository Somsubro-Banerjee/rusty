use std::io;
use std::env;


// a simple cli program ot print your name with your age using command line arguments

fn printUsingCLIArguments() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}