struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
// struct AlwaysEqual; // when you don't have a specific data type

#[derive(Debug)] // if we want to print using {}
struct Car {
    make: String,
    model: String,
}

// methods
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    let sam = build_user(String::from("Samson"), String::from("samson@example.com"));
    let sign_in_count = sam.sign_in_count;
    let username = sam.username;
    let email = sam.email;
    let active = sam.active;

    println!(
        "Username: {username}\nEmail: {email}\nActive:{active}\nSign in count: {sign_in_count}"
    );

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black : {},{},{}", black.0, black.1, black.2);
    println!("origin: {},{},{}", origin.0, origin.1, origin.2);

    //  let subject = AlwaysEqual;

    let my_car = Car {
        make: "Mazda".to_string(),
        model: "Atenza".to_string(),
    };
    println!("My car is a {} {}", my_car.make, my_car.model);
    println!("my car is: {my_car:?}");
    dbg!(&my_car);

    let mut rect = Rectangle {
        width: 0,
        height: 0,
    };
    rect.new(30, 50);
    println!(
        "The area of rectangle is {} and the perimeter is {}",
        rect.area(),
        rect.perimeter()
    );
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
