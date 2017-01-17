#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate pretty_env_logger;

use chrono::*;
use std::{thread, time};
use std::env;
use std::io::Write;
use std::fs::OpenOptions;

lazy_static! {
    #[derive(Copy, Clone, Debug)]
    static ref START: String = Local::now().to_string();
}

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
    pretty_env_logger::init().unwrap();
    let dt = Local::now();
    let mut p = env::current_dir().unwrap();
    p.push(format!("{}.log", dt.format("%Y%m%d")));

    let ten_secs = time::Duration::from_secs(60);
    let filename = p.to_str().unwrap();
    loop {
        let work = Local::now().to_string();
        let mut f = OpenOptions::new().write(true)
            .create(true)
            .open(filename)
            .chain_err(|| "unable to open file")?;
        let mut bytes: String = String::new();
        bytes.push_str("Start work at ".as_ref());
        bytes.push_str(&**START);
        bytes.push_str("\n");
        bytes.push_str("Still work at ".as_ref());
        bytes.push_str(work.as_ref());
        f.write_all(&bytes.into_bytes()).chain_err(|| "unable to write to file")?;
        f.flush().chain_err(|| "unable to flush content")?;
        thread::sleep(ten_secs);
    }
}
