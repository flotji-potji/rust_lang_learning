// Chapter 4 - Understanding Ownership: The Slice Type

pub fn surprise() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("The manual slice of s would be s = {}", &s[..word]);

    s.clear();

    println!("Whereas here the slice of the cleared s would be s = \"\"");

    s = String::from("hello world");

    println!(
        "Here we use automatic slicing for s = {}",
        new_first_word(&s)
    );
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn new_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
