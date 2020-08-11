// with enums we can store different types directly
// (no needs for structs)
// we can also have different data associated with each enum
enum IpAddrKind {
    V4(u32, u32, u32, u32),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self){
        // some method for all Message enums
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    California,
    Michigan
}

// match
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
        // if a quarter gets passed through this function
        // print the state of the quarter
        Coin::Quarter(state) => {
            println!("State quarter from {:#?}", state);
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

fn main() {
    // Enums - Group object types together.
    let four = IpAddrKind::V4(127, 0, 0, 0);
    let six = IpAddrKind::V6(String::from("::1"));
    let m = Message::Write(String::from("hello"));
    m.call();

    // Option types:
    // Option is in place of null types in Rust
    // essentially the Option type is an enum with two types:
    // `Some` type and `None` type. The some type can hold any type of data
    // while the `None` type is a replacement for nulls.
    let some_number = Some(5);
    let some_string = Some("a string");
    let mut absent_number: Option<i32> = None;
    print_option(&absent_number);
    absent_number = Some(3);
    print_option(&absent_number);
    println!("Absent number: {:?}", absent_number);

    // match control flow operator
    let penny = Coin::Penny;
    println!("{}", value_in_cents(penny));

    let california_quarter = Coin::Quarter(UsState::California);
    println!("{}", value_in_cents(california_quarter));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // can match up to a point and use _
    // to cover all cases.
    let some_u8_value = 1;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // if let:
    let some_u8_value = Some(3);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}


fn print_option(x: &Option<i32>){
    // Handling `Option` types in case of 
    // both `Some` type and `None` type
    if x.is_some() {
        println!("X is a `Some` type and it's value is: {}", x.unwrap());
    } else {
        println!("X is null!");
    }
}