use builder_derive::Builder;
use failure::Error;
use failure::err_msg;
use std::result::Result;

#[derive(Default, Builder)]
struct Test {
    x: i64,
}


fn main() {
    println!("Hello, world!");
    println!("{:?}", TestBuilder::default());
    let mut builder = TestBuilder::default();
    builder.x(10);
    let test = builder.build();
}


