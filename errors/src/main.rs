use std::fs::File;
use std::io::{ self, Read };
use std::error::Error;

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn main() -> Result<(), Box<dyn Error>> {
    let username = read_username_from_file()?;
    if let Some(last_letter) = last_char_of_first_line(&username) {
        println!("{last_letter}");
    }
    Ok(())
}
