extern crate redis;
extern crate dotenv;

use redis::Commands;
use dotenv::dotenv;
use std::env;

fn fetch_an_integer<T: AsRef<str>>(uri: T) -> redis::RedisResult<isize> {
    // connect to redis
    let client = try!(redis::Client::open(uri.as_ref()));
    let con = try!(client.get_connection());
    // throw away the result, just make sure it does not fail
    let _: () = try!(con.set("my_key", 42));
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("my_key")
}

fn main() {
    dotenv().ok();
    let host = env::var("REDIS_HOST").unwrap();
    let pass = env::var("REDIS_PASS").unwrap();
    let uri = format!("redis://:{}@{}", pass, host);
    println!("{}", uri);
    let i = fetch_an_integer(uri).unwrap();
    println!("{}", i)
}
