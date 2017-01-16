// http://blog.jwilm.io/from-str-to-cow/
use std::borrow::Cow;
use std::thread;

#[derive(Debug)]
struct Token<'a> {
    raw: Cow<'a, str>,
}

impl<'a> Token<'a> {
    pub fn new<S>(raw: S) -> Token<'a>
        where S: Into<Cow<'a, str>>
    {
        Token { raw: raw.into() }
    }
}

fn main() {

    let raw = String::from("abc");
    let token_owned = Token::new(raw);
    let token_static = Token::new("123");

    thread::spawn(move || {
            println!("token_owned: {:?}", token_owned);
            println!("token_static: {:?}", token_static);
        })
        .join()
        .unwrap();
}
