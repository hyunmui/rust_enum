use std::{
    net::{Ipv4Addr, Ipv6Addr},
    os::macos::raw::stat,
};

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum IpAddrStdlib {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// various type of enum's values
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("run Message.call")
    }
}

// match expression for enum
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:?} / {:?}", home, loopback);

    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));

    println!("{:?} / {:?}", home, loopback);

    let m = Message::Write(String::from("hello?"));

    m.call();

    // Option (Nullable) Enum
    // Some: has value
    let some_number = Some(5);
    let some_string = Some("a string");

    // None: not exist
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // do not working
    // let sum = x + y;

    let sum = x + match y {
        None => 0,
        Some(i) => i,
    };

    println!("{}", &sum);

    // match expression
    value_in_cents(Coin::Penny);

    // match expression with Option
    let six = plus_one(Some(5));
    let none = plus_one(None);

    println!("{:?} / {:?}", six, none);

    // match expression - Catch all pattern
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // like switch's default
    }

    // if-let
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let some_u8_value = Some(0u8) {
        println!("three");
    }

    // match vs if-let
    // let coin = Coin::Quarter(UsState::Alabama);
    let coin = Coin::Nickle;

    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("{:?}주의 25센트 동전!", state),
        _ => count += 1,
    }

    println!("동전 갯수: {}", &count);
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
