use std::env;
use std::io;
use std::io::Read;

mod arguments;
use arguments::{manage_arguments, paste, copy};
use atty::Stream;

fn main() {
    let arg: Vec<String> = env::args().collect();
    let line: String;
    let is_pipe = atty::is(Stream::Stdin);

    // If there is no pipe, execute the default behavior
    if is_pipe {
        if arg.len() > 1 {
            manage_arguments(arg);
        }
        else {
            // TODO
            // Change default dehavior
            paste();
        }
    }
    else {
        // If there is a pipe, copy the content
        line = load_line();
        copy(line);
    }

}

fn load_line () -> String {
    let mut stdin = io::stdin().lock(); 
    let mut line = String::new();

    loop {
        match stdin.read_to_string(&mut line) {
            Ok(n) => {
                if n == 0 { break; }
            }
            Err(err) => {
                eprint!("{}", err);
            }
        }
    }
    line
}
