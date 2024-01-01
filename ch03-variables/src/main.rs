use std::io;

fn main() {
    let x = 5;
    let x = x+1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("The length of spaces is: {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess: {}", guess);

    let quotient = 56.7 / 32.2;
    println!("Quotient: {}", quotient);

    let truncated: i32 = -5/3;
    println!("Truncated: {}", truncated);

    let heart_eyed_cat = '😻';
    println!("I'm printing a heart-eyed cat: {}", heart_eyed_cat);

    let tup = (500, 6.4, 1);
    println!("Tup: {:?}", tup);
    let (x, y, z) = tup;
    println!("Tup: {}, {}, {}", x, y, z);

    let x: (i32,f64,u8) = (500, 6.4,1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("x: {}, {}, {}", five_hundred, six_point_four, one);

    let a = [1,2,3,4,5];
    println!("a: {:?}", a);

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September",
    "October", "November", "December"];
    println!("months: {:?}", months);

    println!("Please enter an array index: ");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index : usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number!");

    let element = a[index];

    println!("The value of the element at index {} is: {}", index, element);

    let x = plus_one(5);
    println!("The value of plus_one is: {}", x);

    let number = if index == 1 {
        println!("number is going to be 5");
        5
    } else {
        println!("number is going to be 6");
        6
    };
    println!("The value of number is: {}", number);

    for i in 0..3 {
        println!("The value of i is: {}", i);
    }

    for i in (4..7).rev() {
        println!("The value of i is: {}", i);
    }

    let mut counter = 0;
    let result = loop {
        counter +=1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The value of result is: {}", result);

}

fn plus_one(x: i32) -> i32 {
    x + 1
}