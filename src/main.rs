use std::env;
use std::io;
use std::io::Read;

mod arguments;
use arguments::{manage_arguments, paste, copy};

fn main() {
    let arg: Vec<String> = env::args().collect();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut line = String::new();
    let mut total_bytes = 0;

    loop {
        match stdin.read_to_string(&mut line) {
            Ok(n) => {
                if n == 0 { break; }
                total_bytes += n;
            }
            Err(err) => {
                eprint!("{}", err);
            }
        }
    }

    if total_bytes == 0 {
        if arg.len() > 1 {
            manage_arguments(arg);
        }
        else {
            paste();
        }
    }
    else { copy(line) }
}
