/*
 * Integration tests call your library like any other piece of code would.
 * And as such, they live in a different directory and only have access
 * to the public items in your library
 */

use adder::add_two;

#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}
