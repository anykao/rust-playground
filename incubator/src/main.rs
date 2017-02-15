extern crate itoa;

use std::borrow::Cow;
use std::io::Write;

#[derive(Debug)]
struct Client<'a> {
    api_key: Cow<'a, str>,
}
impl<'a> Client<'a> {
    fn new<K>(k: K) -> Self
        where K: Into<Cow<'a, str>>
    {
        Client { api_key: k.into() }
    }
}
fn main() {
    let c = Client::new("hello".to_string());

    println!("{:?}", c)
}
