use builder_derive::Builder;

#[derive(Builder)]
struct Test {
    test: i64,
}

fn main() {
    println!("Hello, world!");
    TestBuilder::build();
}
