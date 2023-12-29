enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Four different types of Message structs merged into one Enum
enum Message {
    Quite,
    Move { x: i32, y: i32 }, // Annouimous Struct
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_function() {
        println!("Lets master Rust!");
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

/// Match Expression
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
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
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(usState) => {
            println!("State quarter from {:?}!", usState);
            25
        }
    }
}

fn main() {
    // let four = IpAddrKind::v4;
    // let six = IpAddrKind::v6;

    let localhost = IpAddrKind::V4(127, 0, 0, 1);

    // let localhost = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // Option Enum
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    let some_numer = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap_or(0);

    value_in_cents(Coin::Quarter(UsState::Alaska));

    // Wrapping in match
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    /// IF/else match
    let some_value = Some(3);

    if let Some(3) = some_value {
        println!("Three");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None,
    }
}

fn route(ip_kind: IpAddrKind) {}
