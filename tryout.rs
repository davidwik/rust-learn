fn main() {
    let my_string = String::from("Hello world");

    let ba = &my_string[6..];

    println!("{ba}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
