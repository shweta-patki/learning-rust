/*
Basic Rust functionality

Functions, loops, conditional statements.
*/

fn main() {

    /*Variable declaration and typing*/
    const NEEDS_TYPE: &str = "abc";
    let _autodetect = 53;
    let mut _changeable = 54; //by default variables are const
    let _integerx: i32 = 55;
    let _characterx: char = 'D';
    let _doublex: f64 = 10.45;
    let _boolx: bool = true;

    /*Conditional statements*/
    if _integerx > 55 {
        println!("gt 55")
    }
    else if _integerx < 55 {
        println!("lt 55")
    }
    else {
        println!("eq 55")
    }

    let time = 10;
    let greeting = if time < 18 { "Good day." } else { "Good evening." }; //equivalent to ternary operator

    let day = 4;
    match day {
    1 => println!("Monday"),
    2 => println!("Tuesday"),
    3 => println!("Wednesday"),
    4 => println!("Thursday"),
    5 => println!("Friday"),
    6 => println!("Saturday"),
    7 => println!("Sunday"),
    _ => println!("Invalid day."),
    }

    /*Loops*/
    let mut count = 1;
    loop {
        println!("Looping! {}", count);
        count+=1;
        if count==4{
            break;
        }
    }
    while count <= 5 {
        println!("Count: {}", count);
        count += 1;
    }

    //println!("{:?}", 1..5); //1 2 3 4
    //println!("{:?}", 1..=5); //1 2 3 4 5

    for i in 1..5{
        println!("for {}", i);
    }

}
