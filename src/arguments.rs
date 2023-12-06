extern crate copypasta;
use copypasta::{ClipboardContext, ClipboardProvider};


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
        Option::Copy => {
            if arg.len() == 3 { copy (&arg[2]) }
            else { println! ("Error: Invalid number of arguments", ) }
            
        },
        Option::Paste => {
            if arg.len() > 1 { println! ("Error: Too many arguments", ) }
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

    let mut ctx = ClipboardContext::new().unwrap();

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
    
    let file_content = read_file(file_name).expect("Cannot read file");
    // Get the content of the file

    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(file_content.to_owned()).unwrap();
    // Copy the content to the clipboard
}

fn read_file(file_name: &String) -> Result<String, io::Error> {
    
    let file_content: String;

    file_content = fs::read_to_string(file_name)
                    .expect("Cannot read file");
    // print!("{}", file_content);

    Ok(file_content)
}

