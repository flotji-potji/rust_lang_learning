// Chapter 3 - Common Programming Concepts: Variables and Mutability

const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
const YEARLY_CHOC_CONSUMPTION: u32 = 29 * 6 * 2 * 40 * 365;

pub fn fun() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS} s");
    println!(
        "Number of chocolates consumbed in a year: {}",
        YEARLY_CHOC_CONSUMPTION
    );

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let spaces = "    ";
    println!("This is spaces: {spaces}");
    let spaces = spaces.len();
    println!("This is spaces too: {spaces}, but the length of spaces");
}
