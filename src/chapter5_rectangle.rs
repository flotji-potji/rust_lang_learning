// Chapter 5 - Using Structs to Structure Related Data: An Example Program Using Structs

pub fn start_rectangles() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}