fn first_space_char(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i,&c) in bytes.iter().enumerate() {
        if c == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i,&c) in bytes.iter().enumerate() {
        if c == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn main() {
    let s = String::from("Hello, world! WHOO another world word");
    let first_space = first_space_char(&s);
    println!("first space in {s} is at index {first_space}");

    let word = first_word(&s);
    // reading word len AFTER creating s2 won't work bc s goes out of scope
    // along with any string slice references to it!
    let word_len = word.len();

    let mut s2 = s;
    s2.drain(..word_len + 1);
    println!("{s2}");
    let second_word = first_word(&s2);
    println!("second word is: {second_word}");
}
