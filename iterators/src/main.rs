fn main() {
    let prime_numbers = vec![2, 3, 5, 7, 11, 13, 17, 19, 23];

    for p in prime_numbers.iter() {
        println!("prime number: {:2}", p);
    }

    let mut numbers = vec![1, 2, 3];
    for n in numbers.iter_mut() {
        *n *= 2;
    }
    println!("{:?}", numbers);

    let small_primes: Vec<_> = prime_numbers.iter().filter(|&x| *x < 10).collect();
    println!("{:?}", small_primes);

    let odd_numbers = vec![1, 3, 5, 7, 9];
    let sum: i32 = odd_numbers.iter().fold(0, |acc, &x| acc + x);
    println!("sum of odd numbers is {sum}");

    let even_numbers = vec![0, 2, 4, 6, 8];
    let odd_numbers = vec![1, 3, 5, 7, 9];
    for (e, o) in even_numbers.iter().zip(odd_numbers.iter()) {
        println!("even: {e}, odd: {o}");
    }

    let cars = vec!["Mazda", "Subaru", "Toyota"];
    cars.iter().for_each(|&car| println!("car: {car}"));

    let cars = vec!["Mazda", "Subaru", "Toyota"];
    let cars: Vec<_> = cars.iter().map(|&car| car.to_uppercase()).collect();
    println!("{cars:?}");

    let even_numbers = vec![0, 2, 4, 6, 8];
    println!("take 3");
    even_numbers.iter().take(3).for_each(|n| println!("{n}"));
    println!("skip 2");
    even_numbers.iter().skip(2).for_each(|n| println!("{n}"));
}
