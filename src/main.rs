use std::io;

mod chapter1_getting_started;
mod chapter2_guessing_game;
mod chapter3_variables_mutability;
mod chapter3_data_types;
mod chapter3_functions;
mod chapter3_branches_and_more;
mod chapter3_temperature_converter;
mod chapter4_slices;
mod chapter5_initiating_structs;
mod chapter5_rectangle;
mod chapter6_defining_enums;
mod chapter6_match_control;

fn main() {
    println!("This is my journy through the Rust programming language.");
    println!("Each chapter is organized in its own program");
    println!("Please select a chapter which should be executed:");
    println!("1. \t Chapter 1: Hello World!");
    println!("2. \t Chapter 2: Guessing Game");
    println!("3. \t Chapter 3: Variables and Mutability");
    println!("4. \t Chapter 3: Data Types");
    println!("5. \t Chapter 3: Functions");
    println!("6. \t Chapter 3: Control Flow");
    println!("7. \t Chapter 3: Temperature Converter");
    println!("8. \t Chapter 4: The Slice Type");
    println!("9. \t Chapter 5: Defining Structs");
    println!("10. \t Chapter 5: Rectangle Example");
    println!("11. \t Chapter 6: Defining Enums");
    println!("12. \t Chapter 6: Match Control Flow");

    let mut pick = String::new();

    io::stdin()
        .read_line(&mut pick)
        .expect("Error in handling input!");

    pick = pick.trim().to_string();

    if pick.eq("1") {
        chapter1_getting_started::say_hello();
    } else if pick.eq("2") {
        chapter2_guessing_game::guessing_game();
    } else if pick.eq("3") {
        chapter3_variables_mutability::fun();
    } else if pick.eq("4") {
        chapter3_data_types::data_types();
    } else if pick.eq("5") {
        chapter3_functions::boom();
    } else if pick.eq("6") {
        chapter3_branches_and_more::branches();
        chapter3_branches_and_more::froop_loops();
    } else if pick.eq("7") {
        chapter3_temperature_converter::temperature_fun();
    } else if pick.eq("8") {
        chapter4_slices::surprise();
    } else if pick.eq("9") {
        chapter5_initiating_structs::many_structs();
    } else if pick.eq("10") {
        chapter5_rectangle::start_rectangles();
    } else if pick.eq("11") {
        chapter6_defining_enums::enum_fun();
    } else if pick.eq("12") {
        chapter6_match_control::coin_toss();
    }
}
