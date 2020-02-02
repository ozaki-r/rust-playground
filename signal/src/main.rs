extern crate signal_hook;

use std::io::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{thread, time};

fn main() -> Result<(), Error> {
    let sec = time::Duration::from_secs(1);
    let term = Arc::new(AtomicBool::new(false));
    let start = time::Instant::now();
    let timeout = time::Duration::from_secs(5);

    signal_hook::flag::register(signal_hook::SIGINT, Arc::clone(&term))?;

    loop {
        if term.load(Ordering::Relaxed) {
            println!("SIGINT received");
            break;
        }
        if start.elapsed() > timeout {
            println!("Timed out");
            break;
        }
        println!("waiting");
        thread::sleep(sec);
    }
    Ok(())
}