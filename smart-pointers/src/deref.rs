pub fn run() {
    following_the_pointer_to_the_value();
    using_box_like_a_reference();
    defining_our_own_smart_pointer();
    implicit_deref_coercions();
}

fn following_the_pointer_to_the_value() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn using_box_like_a_reference() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct SmartPointer<T>(T);

impl<T> SmartPointer<T> {
    fn new(x: T) -> SmartPointer<T> {
        SmartPointer(x)
    }
}

use std::ops::Deref;

impl<T> Deref for SmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn defining_our_own_smart_pointer() {
    let x = 5;
    let y = SmartPointer::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn implicit_deref_coercions() {
    let m = SmartPointer::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
