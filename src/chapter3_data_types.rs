// Chapter 3 - Common Programming Concepts: Data Types

pub fn data_types() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    println!("The third value of tup: {}, is equal to z: {}", tup.2, z);
    println!("X: {}, is equal to the first value of tup: {}", x, tup.0);

    let a = [3; 5];
    println!("Five threes in one a: {}", a[3]);
}