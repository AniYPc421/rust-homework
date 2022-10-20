use std::io;

fn main() {
    let mut word = String::new();
    let mut first_char = String::new();
    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read a line!");
    if let Some(x) = word.chars().next() {
        first_char = String::from(x);
    }
    word.remove(0);
    if word.ends_with('\n') {
        word.pop();
    }
    const VOWEL: [&str; 5] = ["a", "e", "i", "o", "u"];
    let mut flag = true;
    for c in VOWEL {
        if c == first_char {
            flag = false;
            break;
        }
    }
    let to_add: String;
    if flag {
        to_add = format!("-{}ay", first_char);
    } else {
        to_add = String::from("-hay");
    }
    println!("{}{}", word, to_add);
}
