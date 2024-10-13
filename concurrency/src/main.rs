mod channels;
mod threads;

fn main() {
    threads::run_threads();

    println!("------------");
    threads::waiting_with_join();

    println!("------------");
    threads::with_closures();

    println!("------------");
    channels::calculations();

    println!("------------");
    channels::multiplevalues();

    println!("------------");
    channels::mutex();
}
