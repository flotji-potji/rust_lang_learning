// Chapter 6 - Enums and Pattern Matching: The match Control Flow Construct
#![allow(unused)]
#[derive(Debug)]

enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn coin_toss() {
    let my_quarter = Coin::Quarter(UsState::Alaska);

    println!("{}", value_in_cents(my_quarter));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
