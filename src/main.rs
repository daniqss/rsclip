use std::env;

mod arguments;
use arguments::*;

fn main() {
    let arg: Vec<String> = env::args().collect();

    println!("{}", arg[0]);

    if arg.len() > 1 {
        
        for i in 1..arg.len() {
            if arg[i] == "-h" || arg[i] == "-help" {
                help();
            } else if arg[i] == "-v" || arg[i] == "-version" {
                version();
            } else {
                println!("error: unexpected argument '{}' found", arg[i]);
                println!("Try --help' for more information.");
                break;
            }
        }
    }
}


