#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    println!("Hello Enum!");
    let first_message = Message::Move { x: 123, y: -32 };
    first_message.call();
}

// Stopped at option
