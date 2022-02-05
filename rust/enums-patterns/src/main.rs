enum IpAddrKind {
    V4, 
    V6,
}

enum IpAddr {
    V4(String),
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
        println!("Testing");
    }
}

// Null equivlent in rust
//enum Option<T>{
//    None, 
//    Some(T),
//}

#[derive(Debug)]
enum Country {
    Australia,
    USA,
    England,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Country),
}

// Demonstrating match functionality
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(country) => {
            println!("A coin from {:?}!", country);
            25
        },
    }
}


// Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    // Enum's 
    let four = IpAddr::V4(String::from("127.0.0.1"));
    let six = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello"));
    m.call();

    let some_num = Some(5);
    let some_str = Some("a string");

    // let absent_number: Option<i32> = None; // Doesn't work
    
    // Match operator
    let coin = Coin::Quarter(Country::Australia);
    let val = value_in_cents(coin);
    println!("Value = {}", val);

    // Option<T> sum
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // if let control flow (similar to match but for smaller cases)
    // Useful for handlng one case and ignoring others
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }



    
}
