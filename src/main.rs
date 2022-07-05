use std::io;

mod chapter1_getting_started;
mod chapter2_guessing_game;

fn main() {
    println!("This is my journy through the Rust programming language.");
    println!("Each chapter is organized in its own program");
    println!("Please select a chapter which should be executed:");
    println!("\t Chapter 1: Hello World!");
    println!("\t Chapter 2: Guessing Game");
    
    let mut pick = String::new();

    io::stdin()
        .read_line(&mut pick)
        .expect("Error in handling input!");
    
    let pick = pick.trim().to_string();

    if pick.eq("1") { 
        chapter1_getting_started::say_hello();
    } else if pick.eq("2") {
        chapter2_guessing_game::guessing_game();
    }
}