// Chapter 6 - Enums and Pattern Matching: Defining an Enum
#![allow(unused)]

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Option<T> {
    None,
    Some(T),
}

pub fn enum_fun() {
    let home = IpAddrKind::V4(127, 0, 0, 1);

    let loopback = IpAddrKind::V6(String::from("::1"));

    let some_number = Some(5);
    let some_string = Some("a string");

    //let absent_number: Option<i32> = None;
}
