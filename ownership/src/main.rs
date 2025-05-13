fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;

    ownership_and_functions();
    return_values_and_scope();
    returning_multiple_values();
}

fn ownership_and_functions() {
    let s = String::from("Hello");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn return_values_and_scope() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn returning_multiple_values() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
