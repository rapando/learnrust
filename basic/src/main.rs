// declare constants
const SECONDS_IN_AN_HOUR: u32 = 60 * 60;

fn main() {
    let mut x = 5; // by making this mutable, we can change the value
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    let hours: u32 = 3;
    let seconds_in_three_hours: u32 = hours * SECONDS_IN_AN_HOUR;
    println!("There are {seconds_in_three_hours} seconds in three hours.");

    // shadowing - basically overwriting a variable especially when converting
    // data type
    let y = 5;
    let y = y + 1; // this y is basically a new variable.
                   // Therefore, the concept of immutability
                   // does not matter here
    println!("The value of y is {y}");

    // another example of shadowing is converting data types
    let name = "samson";
    println!("name is {name}");
    let name = name.len(); // note that we use 'let' here because without it,
                           // it would no longer be shadowing and we would be
                           // stopped with the immutability concept.
    println!("The length of name is {name}");

    let height: f32 = 56.001;
    let width: f32 = 45.78;
    let area = width * height;
    println!("the area is {area}");

    let gender: char = 'M';
    println!("gender is {gender}");

    // tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // to get the individual values
    let (x, y, z) = tup;
    println!("The values in the tup are: {x}, {y} and {z}");
    let first_value = tup.0;
    println!("The first value in the tuple is {first_value}");

    // array
    let ages = [1, 2, 3, 4, 5];
    let first_age = ages[0];
    println!("ages array : {first_age}");

    let ages: [i32; 3] = [1, 2, 3]; // an array with a specific type and length
    let second_age = ages[1];
    println!("second age is {second_age}");

    another_function();
    new_function(45);

    let a = 45;
    let b = add_five(a);
    println!("b={b}");

    if b < 10 {
        println!("b is less than 10");
    } else {
        println!("b is not less than 10");
    }

    let gender = if gender == 'M' { "Male" } else { "Female" };
    println!("gender is now {gender}");

    for i in 1..100 {
        if i % 7 == 0 {
            println!("{i}");
        }
        if i == 28 {
            continue;
        }
        if i == 55 {
            break;
        }
    }

    let mut counter = 0;
    while counter < 10 {
        println!("number: {counter}");
        counter += 1;
    }

    let number = 4;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        _ => println!("Not 1,2,3 or 4"),
    }

    // ownership
    // a variable can have one owner at a time and when the owner goes out of
    // scope, it is freed.
    // Rust allows borrowing which can be mutable or immutable
    // You can have multiple immutable references but only one mutable.
    //
    let s1 = String::from("Hello");
    let s2 = s1; // ownership moves from s1 to s2
    println!("{}", s2);

    let s1 = String::from("hello"); // remember shadowing
    let s3 = &s1; // borrowing s1
    println!("s1={}, s3={}", s1, s3);
}

fn another_function() {
    println!("This is another function, called by reference");
}

fn new_function(x: i32) {
    println!("New function takes parameters: {x}");
}

fn add_five(x: i32) -> i32 {
    return x + 5;
}
