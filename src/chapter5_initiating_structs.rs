// Chapter 5 - Using Structs to Structure Related Data: Defining and Instantiating Structs

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

pub fn many_structs() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!(
        "{} {} {} {}",
        user2.active, user2.email, user2.sign_in_count, user2.username
    );

    let mut user3 = build_user(String::from("my@example.com"), String::from("hello"));
    user3.active = false;

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", black.0);
    println!("{}", origin.0);

    
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
