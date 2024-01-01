fn main() {
    let s1 = "hello";
    let _s2 = s1;

    println!("{s1}, {_s2}");

    let ow = gives_ownership();
    println!("{}",ow);

    let s: String = String::from("value will be moved");
    takes_ownership(s);
    // println!("{s}");  // this won't compile as s1 has been moved

    let s: String = String::from("hello");
    let (s2,len) = calculate_length(s);
    println!("The length of {s2} is {len}");

    let s: String = String::from("hello");
    let len: usize = calculate_length2(&s);
    println!("The length of {s} is {len}");

    let i: i32 = i32::MAX; 
    println!("{i}");

    let i: i32 = 2147483600;
    let j: i32 = 2147483500;
    let k: i32 = i + j;
    println!("{k}");
}

fn calculate_length(s: String) -> (String, usize) {
    let len: usize = s.len();
    (s, len)
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

fn takes_ownership(s: String) {
    println!("{s}");
}

fn gives_ownership() -> String {
    let mut  s = String::from("hello");
    s.push_str("test");
    return s;
}