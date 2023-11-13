use std::fs::File;
use std::io::{self, Read};

pub fn help() {
    println!("Usage: ./main [OPTION]... [FILE]...");
    println!("Copy to clipboard the contents of FILE, or standard input if none.");
    println!("");
    println!("  -h, --help     display this help and exit");
    println!("  -v, --version  output version information and exit");
    println!("");
}

pub fn version () {
    println!("version la q sea");
}

pub fn read_input(file_name: String) -> String {
    let file_content = read_input(file_name);

    match file_content {
        Ok(file_content) => {
            println!("Contenido del archivo:\n{}", file_content);
            file_content
        }
        Err(e) => {
            eprintln!("Error al leer el archivo: {}", e);
            "Error"
        }
    }
}

fn read_file(file_name: String) -> Result<String, io::Error> {
    let mut file = File::open(&file_name)?;
    let mut file_content = String::new();

    file.read_to_string(&mut file_content);


    Ok(file_content)
}