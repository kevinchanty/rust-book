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

#[derive(Debug)]
enum UsState {
    Who,
    Cares,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}

fn add_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(number) => Some(number + 1),
        None => None,
    }
}

fn main() {
    println!("Hello Enum!");
    let first_message: Message = Message::Move { x: 123, y: -32 };
    first_message.call();

    let some_number = Some(32);
    let absent_number: Option<i32> = None;

    value_in_cents(Coin::Quarter(UsState::Who));

    let five = Some(5);
    let six = add_one(five);
    let none = add_one(None);

    dbg!(six);

    let dice_roll = 9u8;
    match dice_roll {
        3 => println!("Haha"),
        _ => (),
        // other => {dbg!(other);}
    }

    let some_result = Some(String::from("success la"));
    if let Some(res) = some_result {
        println!("{res}");
    };
}

// Stopped at if let
