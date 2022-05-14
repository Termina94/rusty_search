use core::fmt::Debug;
use std::fmt::Display;

#[allow(dead_code)]
fn header() {
    println!("\n##### DEBUG #####");
}

#[allow(dead_code)]
pub fn text<S>(text: S)
where
    S: Into<String>,
    S: Display,
{
    header();
    println!("{}", text);
}

#[allow(dead_code)]
pub fn obj<T>(object: T)
where
    T: Debug,
{
    header();
    println!("{:?}", object)
}

#[allow(dead_code)]
pub fn text_obj<S, T>(text: S, object: T)
where
    S: Into<String>,
    S: Display,
    T: Debug,
{
    header();
    println!("{} - {:?}", text, object)
}
