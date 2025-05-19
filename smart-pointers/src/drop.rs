pub fn run() {
    running_code_cleanup_with_drop();
    calling_drop_early();
}

struct SmartPointer {
    data: String,
}

impl Drop for SmartPointer {
    fn drop(&mut self) {
        println!("Dropping SmartPointer with data `{}`!", self.data);
    }
}

fn running_code_cleanup_with_drop() {
    let a = SmartPointer {
        data: String::from("my stuff"),
    };
    let b = SmartPointer {
        data: String::from("other stuff"),
    };
    println!("SmartPointers created.");
}

fn calling_drop_early() {
    let c = SmartPointer {
        data: String::from("some data"),
    };
    println!("SmartPointer created.");
    drop(c);
    println!("SmartPointer dropped before the end of main.");
}
