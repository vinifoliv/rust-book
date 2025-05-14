pub fn run() {
    dangling_reference();
    generic_lifetimes_in_functions();
    restrict_lifetimes();
    lifetime_annotations_in_struct_definitions();
    lifetime_in_method_definition();
    static_lifetime();
    mixing_up();
}

fn dangling_reference() {
    let x = 5;
    let r = &x;

    println!("r: {r}");

    // let r;
    //
    // {
    //     let x = 5;
    //     r = &x;
    // }
    //
    // println!("r: {r}");
}

fn generic_lifetimes_in_functions() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }

    y
}

fn restrict_lifetimes() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn lifetime_annotations_in_struct_definitions() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
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

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention, please: {announcement}");
        self.part
    }
}

fn lifetime_in_method_definition() {}

fn static_lifetime() {
    let s: &'static str = "I have a static lifetime.";
}

use std::fmt::Display;

fn mixing_up() {}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
