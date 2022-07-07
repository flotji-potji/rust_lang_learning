// Chapter 3 - Common Programming Concepts: Fahrenheit/Celsius converter

use std::io;

const CELSIUS_FAHRENHEIT_CONVERSION_RATE: f64 = 1.8;

fn fahrenheit_to_celsius(temp: &f64) -> f64 {
    (temp - 32.0) / CELSIUS_FAHRENHEIT_CONVERSION_RATE
}

fn celsius_to_fahrenheit(temp: &f64) -> f64 {
    (temp * CELSIUS_FAHRENHEIT_CONVERSION_RATE) + 32.0
}

pub fn temperature_fun() {
    println!("Welcome to the awesome Celsius-Fahrenheit Converter!");

    let mut pick = String::new();
    let mut option: u8;

    loop {
        pick = String::new();

        println!("Please select the appropriate option for you:");
        println!("1. \t Fahrenheit to Celsius");
        println!("2. \t Celsius to Fahrenheit");
        println!("3. \t Exit program");

        io::stdin()
            .read_line(&mut pick)
            .expect("Failed to read line!");

        option = match pick.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if option == 1 {
            loop {
                pick = String::new();
                println!("Enter the Fahrenheit value as a floating point number: ");
                io::stdin()
                    .read_line(&mut pick)
                    .expect("Failed to read line!");

                let pick: f64 = match pick.trim().parse() {
                    Ok(num) => {
                        if num > -459.67 {
                            num
                        } else {
                            println!("Incorrect value entered!");
                            continue;
                        }
                    }
                    Err(err_msg) => {
                        println!("{}", err_msg);
                        continue;
                    }
                };
                println!(
                    "Your entered value is equivilant to: {}℃",
                    fahrenheit_to_celsius(&pick)
                );
                break;
            }
        } else if option == 2 {
            loop {
                pick = String::new();
                println!("Enter the Celsius value as a floating point number: ");
                io::stdin()
                    .read_line(&mut pick)
                    .expect("Failed to read line!");

                let pick: f64 = match pick.trim().parse() {
                    Ok(num) => {
                        if num > -273.15 {
                            num
                        } else {
                            println!("Incorrect value entered!");
                            continue;
                        }
                    }
                    Err(_) => continue,
                };
                println!(
                    "Your entered value is equivilant to: {}℉",
                    celsius_to_fahrenheit(&pick)
                );
                break;
            }
        } else if option == 3 {
            break;
        }
    }
}
