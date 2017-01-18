// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate pretty_env_logger;
extern crate dotenv;

use dotenv::dotenv;
use reqwest::header::{Authorization, Bearer};
use std::env;

mod errors {
    error_chain!{}
}
use errors::*;

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
    pretty_env_logger::init().unwrap();
    let api_key = env::var("API_KEY").chain_err(|| "unable to get API_KEY")?;
    let end_point = env::var("END_POINT").chain_err(|| "unable to get END_POINT")?;
    info!("api_key = {}", api_key);
    info!("end_point = {}", end_point);
    let client = reqwest::Client::new().unwrap();
    let mut res = client.get(&end_point)
        .header(Authorization(Bearer { token: api_key.to_owned() }))
        .send()
        .chain_err(|| "unable to send body")?;
    ::std::io::copy(&mut res, &mut ::std::io::stdout()).chain_err(|| "unable to copy to stdout.")?;
    Ok(())
}
