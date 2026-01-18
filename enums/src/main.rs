fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Hello, world!");

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('a');
    let none_at_all: Option<i32> = None;

    value_in_cents(Coin::Quarter(UsState::Alabama));

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}

enum IpAddr {
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
    fn call(&self) {}
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(numb) => Some(numb + 1),
        None => None,
    }
}

fn board_action(dice_roll: u8) {
    match dice_roll {
        3 => println!("add fancy hat"),
        7 => println!("remove fancy hat"),
        // other => println!("move the player"),
        _ => print!("add custom not using value"),
    }
}
