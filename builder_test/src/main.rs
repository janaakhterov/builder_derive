use builder::Builder;
use builder_derive::Builder;

#[derive(Builder)]
struct Test {}

fn main() {
    println!("Hello, world!");
    Test::builder();
}
