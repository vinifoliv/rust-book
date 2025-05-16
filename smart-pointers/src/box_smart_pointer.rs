pub fn run() {
    using_box_to_store_on_the_heap();
    enabling_recursive_types();
}

fn using_box_to_store_on_the_heap() {
    let b = Box::new(0);
    println!("b = {b}");
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::*;

fn enabling_recursive_types() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
