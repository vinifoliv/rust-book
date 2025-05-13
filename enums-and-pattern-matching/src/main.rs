enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // Method body
    }
}

fn main() {
    enum_values();
    methods_in_enums();
    option_enum();
}

fn enum_values() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    route(home);
    route(loopback);
}

fn route(ip_kind: IpAddrKind) {}

fn methods_in_enums() {
    let message = Message::Write(String::from("Hello"));
    message.call();
}

fn option_enum() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}
