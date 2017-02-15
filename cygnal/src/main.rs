#![recursion_limit = "1024"]
extern crate libc;
#[macro_use]
extern crate log;
extern crate futures;
extern crate tokio_core;
extern crate tokio_signal;
#[macro_use]
extern crate error_chain;

use futures::stream::Stream;
use tokio_core::reactor::Core;
use tokio_signal::unix::Signal;

mod errors {
    error_chain!{}
}

use errors::*;

quick_main!(run);

fn run() -> Result<()> {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let signal = core.run(Signal::new(libc::SIGINT, &handle)
            .map(|x| x.map(|_| ()).boxed())
            .boxed())
        .unwrap();
    core.run(signal.into_future()).ok().unwrap();
    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("hello, world");
    }
}
