// Chapter 5 - Using Structs to Structure Related Data: An Example Program Using Structs

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(self: &Self) -> bool {     // "self: &Self" same as "&self"
        self.width > 0
    }
}

pub fn start_rectangles() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    println!("rect1 is: {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
