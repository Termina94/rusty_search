use core::fmt::Debug;
use std::fmt::Display;

fn header() {
    println!("\n##### DEBUG #####");
}

pub fn text<S>(text: S)
where
    S: Into<String>,
    S: Display,
{
    header();
    println!("{}", text);
}

pub fn obj<T>(object: T)
where
    T: Debug,
{
    header();
    println!("{:?}", object)
}

pub fn text_obj<S, T>(text: S, object: T)
where
    S: Into<String>,
    S: Display,
    T: Debug,
{
    header();
    println!("{} - {:?}", text, object)
}
