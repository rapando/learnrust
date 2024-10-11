fn main() {
    println!("Hello, world!");
    eprintln!("Printed on stderr");
    println!("Base 2 (binary) {:b}", 1000);

    // right justify text with a specified width
    println!("A number {n:>5}", n=1);
    println!("A number {n:>5}", n=99_999);
    println!("A number {n:>5}", n=999);

    // you can pad with leading 0s
    println!("A number {n:0>5}", n=999);



}
