// Chapter 5 - Using Structs to Structure Related Data: An Example Program Using Structs

struct Rectangle {
    width: u32,
    height: u32,
}

pub fn start_rectangles() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}