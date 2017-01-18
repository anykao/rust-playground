
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
extern crate sendgrid;

use dotenv::dotenv;
use std::collections::HashMap;
use errors::*;
use reqwest::header::{Authorization, Bearer};
use std::env;
use sendgrid::EmailBuilder;

mod errors {
    error_chain!{}
}

fn main() {
    if let Err(ref e) = run() {
        use ::std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    dotenv().ok();
    env_logger::init().unwrap();
    let api_key = env::var("SENDGRID_API_KEY").chain_err(|| "unable to get SENDGRID_API_KEY")?;
    debug!("{}", api_key);
    let mut map = HashMap::new();
    map.insert("to", "rust");
    map.insert("from", "json");
    map.insert("Subject", "json");
    map.insert("content", "json");

    // let mut builder = EmailBuilder::new().add_subject("".to_owned()).finish();
    let mut builder = EmailBuilder::new();
    let mail = builder.add_from(Some("hesdf".to_owned()), "sdf".to_owned())
        .add_subject("".to_owned())
        .finish();
    println!("{:?}", mail);
    let client = reqwest::Client::new().unwrap();
    let res = client.post("https://api.sendgrid.com/v3/mail/send")
        .header(Authorization(Bearer { token: api_key.to_owned() }))
        .json(&map)
        .send()
        .chain_err(|| "unable to send body")?;

    Ok(())
}
