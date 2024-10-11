#![allow(dead_code)]

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: usize) -> usize {
    a + 2
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    //#[test]
    //fn another() {
    //    panic!("Make this test fail");
    //}

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(
            !smaller.can_hold(larger),
            "Failed because smaller can hold larger. Which should not happen"
        );
    }

    #[test]
    fn it_adds_two() {
        let result = add_two(5);
        assert_eq!(result, 7);
    }

    // in this case, if it panics then we're ok because that is the expected behaviour
    #[test]
    // #[should_panic]
    #[should_panic(expected = "must be between 1 and 100, ")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[ignore] // ignore this test unless specified. For example if its expensive or takes long to
              // run
              // cargo test -- --ignored
    fn it_works2() -> Result<(), String> {
        let result = add(2, 3);
        if result == 5 {
            Ok(())
        } else {
            Err(String::from("two plus three does not equal five"))
        }
    }
}
