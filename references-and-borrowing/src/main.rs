fn main() {
    references_and_borrowing();
    mutable_references();
    // dangling_references();
}

fn references_and_borrowing() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}");
}
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn mutable_references() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

// fn dangling_references() {
//     let reference_to_nothing = dangle();
// }
//
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
