/*
 * Closures     : a function-like construct that you can store as a variable
 * Iterators    : a way of processing a series of elements
 * How to use closures and iterators to improve I/O project in chapter 12
 * The performance of closures and iterators (they are faster)
 */

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    // we're passing self.most_stocked() as a parameter to unwrap_of_else
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
        // same as the two lines below
        /*
         * let ms = self.most_stocked();
         * user_preference.unwrap_or_else(|| ms)
         *
         */
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

// returning closures from a function
fn create_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

// passing closures as function parameters
// when we call apply_to_five, we give it a closure that takes an i32 and returns an i32.
// in turn apply_to_five calls the closure and returns the value
fn apply_to_five<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(5)
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {user_pref1:?} gets {giveaway1:?}");

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {user_pref2:?} gets {giveaway2:?}");

    let add = |x, y| x + y; // a closure
    let result = add(2, 3);
    println!("result is {result}");

    // closures can capture environment variables
    let z = 10;
    let add_to_z = |x| x + z;
    println!("add to z: {}", add_to_z(5));

    // we are using create_adder to create closures
    let add_five = create_adder(5);
    println!("{}", add_five(4));

    let square = |x| x * x;
    let double = |x| x * 2;

    println!("{}", apply_to_five(square)); // square will be given 5 as a parameter
    println!("{}", apply_to_five(double)); // double will be given 5 as a parameter

   }
