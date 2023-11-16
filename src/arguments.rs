// mod clipboard;
// use clipboard::*;

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
        Option::Help => help_opt (),
        Option::Version => version_opt (),
        Option::Copy => {
            if arg.len() == 3 { copy_opt (arg[2]) }
            else { println! ("Error: Invalid number of arguments", ) }
            
        },
        Option::Paste => {
            if arg.len() > 1 { println! ("Error: Too many arguments", ) }
            else { paste_opt () }
        },
        Option::Error (msg) => println! ("Error: {}", msg),
      };

}


fn help_opt() {
    println!("Usage: ./main [OPTION]... [FILE]...");
    println!("Copy to clipboard the contents of FILE, or standard input if none.");
    println!("");
    println!("  -h, --help     display this help and exit");
    println!("  -c, --copy     copy to clipboard the contents of file passed as argument");
    println!("  -p, --paste    paste the last copied element");
    println!("  -i, --info     Display information about the last copied element");
    println!("");
}

fn version_opt () {
    println!("version");
}

fn paste_opt() {
    println!("paste");
}

fn copy_opt(file_name: String) {
    println!("copy");
}

