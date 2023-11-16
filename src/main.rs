use std::env;

mod arguments;
use arguments::manage_arguments;
mod clipboard;
use clipboard::paste;

fn main() {
    let arg: Vec<String> = env::args().collect();

    if arg.len() > 1 {
        manage_arguments(arg);
    }
    else {
        paste();
    }
}


