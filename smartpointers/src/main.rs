/*
 * Smart Pointers aredata structures that act like a pointer but also have other attributes like
 * metadata etc.
 * In Rust, references (&) borrow data but smart pointers own the data. They include String and
 * Vec<T>
 * Smart Pointers are usually implemented using Structs. They implement Deref and Drop traits
 * Deref trait allows an insrance of the smart pointer to behave like a reference.
 * The Drop trait allows you to customize the code that's run when an instance of the smart pointer
 * goes out of scope.
 * Many libraries have their own and you can create yours, but these are the most common:
 * * Box<T> for allocating values in a heap
 * * Rc<T> a reference counting type that enables multiple ownershop
 * * Ref<T> and RefMut<T> accessed through RefCell<T>: a type that enforces the borrowing rules at
 * runtime instead of compile time
 *
 * Interior mutability pattern where an immutable type exposes an API for mutating an interior
 * value.
 * Reference Cycles: how they can leak memory and how to prevent them.
 *
 * ***** 1. Box<T>
 * Box<T> stores data in a heap. What remains on the stack is the pointer to the data. Use them in
 * the following situations:
 * - WHen the type is not known at compile time.
 * - Have a large amount of data and what to transfer ownership without copying the data.
 * - Own a value and care only that it's a type that implements a particular trait rather than
 * being a specific type.
 *
 */

// To determine how much memory Message requires, Rust goes through each element.
// Quit does not need any space
// Move needs enough space to store two i32 and so forth
/*
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
*/

#![allow(dead_code, unused_variables, unused_imports)]

use std::ops::Deref;
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// implement a Deref trait for our box
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
use std::rc::Rc;

enum RList {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::RList::{Cons, Nil};
fn main() {
    let b = Box::new(5); // this is stored in the heap but its reference stored in stack
    println!("b={b}");
    println!("b={b}");

    // recursive types can have another value of the same type as part of itself.
    // sample cons (lisps equivalent to linked lists)
    // (1, (2, (3, Nil)))
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(2, Box::new(Nil))))));

    // Treating Smart Pointers like Regular References with the Deref trait
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // assert_eq!(5, y); // this does not work because they are of differnt types.
    //
    let x = 5;
    let y = Box::new(x); // y is now an instance of Box<T>, pointing to a copied value of x rather
                         // than x itself.
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // we have implemented the ability to dereference our smart pointer by
                       // implementing the Deref trait and telling the compiler that when we use
                       // the deref, it calls deref.
                       // therefore, *y is same as *(y.deref())
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // this works even though the expected and given types are not same
               // MyBox implements the deref trait therefore returns &str (refer top of page)
               // &MyBox<String> -> &String -> &str

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    drop(c); // we can drop it early
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers Created");

    /*
     * Rc<T>, the reference counted smart pointer
     * Useful in cases where a single value might have multiple owners e.g in a graph data
     * structure, multiple edges might point to the same node.
     */
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

fn hello(name: &str) {
    println!("Hello {name}!");
}
