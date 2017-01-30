#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate pretty_env_logger;
extern crate futures;
extern crate tokio_core;
extern crate tokio_signal;

use chrono::*;
use std::thread::{self, sleep};
use std::time::Duration;
use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::fs::OpenOptions;
use futures::stream::Stream;
use tokio_core::reactor::Core;
use tokio_signal::unix::{Signal, SIGTERM};
use std::sync::Arc;

lazy_static! {
    #[derive(Copy, Clone, Debug)]
    static ref START: String = Local::now().to_string();
}

mod errors {
    error_chain!{}
}

use errors::*;

quick_main!(run);

fn write_log<T: Into<String>>(fname: T, is_end: bool) -> Result<()> {
    let work = Local::now().to_string();
    let mut f = OpenOptions::new().write(true)
        .create(true)
        .open(fname.into())
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
    let filename = Arc::new(p);
    let rc1 = filename.clone();

    thread::spawn(move || {
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let cygnal = Signal::new(SIGTERM, &handle);
        let stream = core.run(cygnal).unwrap();
        core.run(stream.for_each(|_| {
                write_log(rc1.to_str().unwrap(), true).unwrap();
                Ok(())
            }))
            .unwrap();
        ()
    });

    let ten_secs = Duration::from_secs(60);
    loop {
        write_log(filename.to_str().unwrap(), false).chain_err(|| "Error in writing log file")?;
        sleep(ten_secs);
    }
}
