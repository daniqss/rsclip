use std::io;
use std::fs;


pub fn copy() {
    // Copy to clipboard the contents of file passed as argument
    println!("copy👮");
}

pub fn paste() {
    // paste the last copied element
    println!("paste🥛");

}


fn read_file(file_name: String) -> Result<String, io::Error> {
    
    //let file_path = File::open(&file_name)?;
    let file_content: String;

    file_content = fs::read_to_string(file_name)
                    .expect("Cannot read file");
    print!("{}", file_content);

    Ok(file_content)
}



fn write_pasted() {
    // Write in a json file information about the last copied element
    println!("write pasted🍝");

} 






