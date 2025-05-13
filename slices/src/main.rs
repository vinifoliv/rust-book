fn main() {
    println!("Hello, world!");
    string_slices();
}

fn string_slices() {
    let s = String::from("Gwynevere, daughter of Gwyn");
    let slice_of_s = first_word(&s);
    println!("{slice_of_s}");
}

fn first_word(some_string: &str) -> &str {
    let bytes = some_string.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &some_string[..i];
        }
    }
    &some_string[..]
}
