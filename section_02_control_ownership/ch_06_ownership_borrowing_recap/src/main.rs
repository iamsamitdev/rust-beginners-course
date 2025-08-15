mod borrowing;
mod lifetime;
mod ownership;

fn main() {
    println!("-- Ownership Example --");
    ownership::sample::ownership_example();

    println!("-- Borrowing Example --");
    borrowing::sample::borrowing_example();

    println!("-- Lifetime Example ");
    lifetime::sample::lifetime_example();
}
