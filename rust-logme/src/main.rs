#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate pretty_env_logger;
extern crate nix;
extern crate signal;


use chrono::*;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::fs::OpenOptions;
use nix::sys::signal::SIGTERM;
use signal::trap::Trap;

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

fn write_log(fname: &str, is_end: bool) -> Result<()> {
    let work = Local::now().to_string();
    let mut f = OpenOptions::new().write(true)
        .create(true)
        .open(fname)
        .chain_err(|| "unable to open file")?;
    //{let file = BufReader::new(&f);
    //{let count = file.lines().count();
    //{debug!("count:{:?}", count);
    //
    let mut contents: String = String::new();
    contents.push_str("Start work at ".as_ref());
    contents.push_str(&**START);
    contents.push_str("\n");
    if is_end {
        contents.push_str("End work at ".as_ref());
        contents.push_str(work.as_ref());
        contents.push_str("\n");
    } else {
        contents.push_str("Still work at ".as_ref());
        contents.push_str(work.as_ref());
        contents.push_str("\n");
    }
    debug!("{}", &contents);
    f.write_all(&contents.into_bytes()).chain_err(|| "unable to write to file")?;
    f.flush().chain_err(|| "unable to flush content")?;
    Ok(())
}

fn run() -> Result<()> {
    pretty_env_logger::init().unwrap();
    let dt = Local::now();
    let mut p = env::current_dir().unwrap();
    p.push(format!("{}.log", dt.format("%Y%m%d")));
    let ten_secs = Duration::from_secs(60);
    let filename = p.to_str().unwrap();
    let trap = Trap::trap(&[SIGTERM]);
    loop {
        if let Some(SIGTERM) = trap.wait(Instant::now()) {
            write_log(filename, true).chain_err(|| "Error in writing log file")?;
            break;
        }
        write_log(filename, false).chain_err(|| "Error in writing log file")?;
        sleep(ten_secs);
    }
    Ok(())
}
