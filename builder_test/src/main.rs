use builder_derive::Builder;
use failure::Error;
use failure::err_msg;
use std::result::Result;

#[derive(Default, Debug, Builder)]
struct Test {
    x: i64,
    y: u8,
    z: String,
}


fn main() {
    println!("Hello, world!");
    println!("{:?}", TestBuilder::default());
    let mut builder = TestBuilder::default();
    builder
        .x(10)
        .y(145)
        .z("ahah".to_string());
    println!("{:?}", builder); 
    let test = builder.build();
    println!("{:?}", test); 
}


