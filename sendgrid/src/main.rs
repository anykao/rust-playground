
// Simple and robust error handling with error-chain!
// Use this as a template for new projects.

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

// Import the macro. Don't forget to add `error-chain` in your
// `Cargo.toml`!
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate env_logger;
extern crate dotenv;


// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

use dotenv::dotenv;
use std::collections::HashMap;
use errors::*;
use reqwest::header::{Authorization, Bearer};
use std::env;

fn main() {
    if let Err(ref e) = run() {
        use ::std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

// Use this macro to auto-generate the main above. You may want to
// set the `RUST_BACKTRACE` env variable to see a backtrace.
// quick_main!(run);


// Most functions will return the `Result` type, imported from the
// `errors` module. It is a typedef of the standard `Result` type
// for which the error type is always our own `Error`.
fn run() -> Result<()> {
    dotenv().ok();
    env_logger::init().unwrap();
    let api_key = env::var("SENDGRID_API_KEY").chain_err(|| "unable to get SENDGRID_API_KEY");
    debug!(api_key);
    let mut map = HashMap::new();
    map.insert("to", "rust");
    map.insert("from", "json");
    map.insert("Subject", "json");
    map.insert("content", "json");

    let client = reqwest::Client::new().unwrap();
    let res = client.post("https://api.sendgrid.com/v3/mail/send")
        .header(Authorization(Bearer { token: api_key.to_owned() }))
        .json(&map)
        .send()
        .chain_err(|| "unable to send body")?;



    // use std::fs::File;


    /// / This operation will fail
    /// File::open("tretrete").chain_err(|| "unable to open tretrete file")?;

    Ok(())
}
