use std::borrow::Borrow;
use std::fmt::Display;

fn foo<T: Borrow<i32> + Display>(a: T) {
    println!("a is borrowed: {}", a);
}

fn is_hello<T: AsRef<str>>(s: T) {
    println!("s is AsRef: {}", s.as_ref());
}

fn main() {
    let mut i = 5;

    foo(i);
    foo(&mut i);

    let s = "hello";
    is_hello(s);

    let s = "hello".to_string();
    is_hello(s);
}
