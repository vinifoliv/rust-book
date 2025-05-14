fn panicking() {
    panic!("Crash and burn!");
}

fn bug_from_library() {
    let v = vec![1, 2, 4];
    v[99]
}
