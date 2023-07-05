fn main() {
    let mut s = String::from("Hello World");

    let word: &str = first_word(&s);

    println!("the first word is: {}", word);

    let s1 = &mut s;
    s1.push_str("string");
    println!("{s}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
