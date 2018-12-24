use builder_derive::Builder;

#[derive(Default, Builder)]
struct Test {
    x: i64,
    y: i64,
    z: i64,
}

fn main() {
    println!("Hello, world!");
    let mut builder = TestBuilder::default();
    builder.y(10);
    println!("{:?}", TestBuilder::default());
}
