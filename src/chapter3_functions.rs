// Chapter 3 - Common Programming Concepts: Functions

pub fn boom() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    println!("The value of x is: {}", plus_one(5));
}

fn plus_one(x: i32) -> i32 {
    x + 1
}