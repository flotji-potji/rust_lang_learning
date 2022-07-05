use std::io;

mod rust_beginning;

fn main() {
    println!("This is my journy through the Rust programming language.");
    println!("Each chapter is organized in its own program");
    println!("Please select a chapter which should be executed:");
    println!("\t Chapter 1: Hello World!");
    
    let mut pick = String::new();

    io::stdin()
        .read_line(&mut pick)
        .expect("Error in handling input!");
    
    if pick.trim().eq("1") { 
        rust_beginning::say_hello();
    }
}