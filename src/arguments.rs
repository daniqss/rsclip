extern crate copypasta_ext;
use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;

use std::io;
use std::fs;

enum Option {
    Help,
    Version,
    Copy,
    Paste,
    Error (String),
}
  
pub fn manage_arguments(arg: Vec<String>) {
    
    let opt = if !arg[arg.len() - 1].starts_with('-') {
        Option::Copy
    } else {
        match arg[1].as_str() {
            "-h" | "--help" => Option::Help,
            "-v" | "--version" => Option::Version,
            "-c" | "--copy" => Option::Copy,
            "-p" | "--paste" => Option::Paste,
            _ => Option::Error("Invalid option".to_string()),
        }
    };


    match opt {
        Option::Help => help (),
        Option::Version => version (),
        Option::Copy => copy(&arg[arg.len() - 1]),
        Option::Paste => {
            if arg.len() > 2 { println! ("Error: Too many arguments", ) }
            else { paste () }
        },
        Option::Error (msg) => println! ("Error: {}", msg),
      };

}


fn help() {
    println!("Usage: ./main [OPTION]... [FILE]...");
    println!("Copy to clipboard the contents of FILE, or standard input if none.");
    println!("");
    println!("  -h, --help     display this help and exit");
    println!("  -c, --copy     copy to clipboard the contents of file passed as argument");
    println!("  -p, --paste    paste the last copied element");
    println!("  -i, --info     Display information about the last copied element");
    println!("");
}

fn version () {
    const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

    println!("Version: {}", PKG_VERSION);
}

/// Paste the clipboard content
///
/// # Examples
///
/// ``` bash
/// rclip -p
/// Your clipboard content
/// 
/// ```

pub fn paste() {

    let mut ctx = copypasta_ext::try_context().expect("Failed to get clipboard context");

    let clipboard_content = ctx.get_contents().unwrap();
    print!("{}", clipboard_content);
    // write_selection_data();

}


/// Paste the clipboard content
///
/// # Examples
///
/// ``` bash
/// rclip -c file.txt
/// 
/// ```

fn copy(file_name: &String) {
    
    let file_content = match read_file(file_name) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let mut ctx = ClipboardContext::new().unwrap();

    ctx.set_contents(file_content.to_owned()).unwrap();
    // Copy the content to the clipboard
}

fn read_file(file_name: &String) -> Result<String, io::Error> {
    match fs::read_to_string(file_name) {
        Ok(file_content) => Ok(file_content),
        Err(err) => Err(err),
    }
}

