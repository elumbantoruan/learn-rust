use std::sync::OnceState;

fn main() {
    let home = IpAddr::V4(127,0,0,1);
    let loopback: IpAddr = IpAddr::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number:Option<i32> = None;

    println!("{:?}", some_number);
    println!("{:?}", some_char);
    println!("{:?}", absent_number);

    let coin : Coin = Coin::Penny;
    println!("{:?}", value_in_cents(coin));

    let five = Some(5);
    let six = plus_one(five);
    println!("{:?}", six);
    let none = plus_one(None);
    println!("{:?}", none);

   let mut coin = Coin::Penny;
   let mut count = 0;
   match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        _ => count += 1,
   }
   println!("count {}", count);

   coin = Coin::Quarter(USState::Washington);
   match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        _ => count += 1,
   }
   println!("count {}", count);

   // use if let
   // shadow coin from above
   let coin = Coin::Quarter(USState::Washington);
   if let Coin::Quarter(state) = coin {
       println!("State quarter from {:?} -- print using if let", state);
   } else {
        count += 1;
   }
   println!("count {}", count);
   

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}



#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("call {:?}", &self);
    }
}

#[derive(Debug)]
enum USState {
    California,
    Washington,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("penny is selected");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_state) => 25,
    }
}

