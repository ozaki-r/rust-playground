use std::env;

extern crate pcap;

use pcap::Error;

fn capture(ifname: &String, filter: &String) -> Result<(), Error> {
    println!("Start capturing.");
    let mut cap = pcap::Capture::from_device(ifname.as_str())
        .unwrap()
        .promisc(true)
        .buffer_size(1024)
        //.timeout(300)
        //.snaplen(100)
        .open()
        .unwrap();
    cap.filter(filter.as_str()).unwrap();
    while let Ok(packet) = cap.next() {
        println!("received packet! {:?}", packet);
    }
    Ok(())
}
fn main() {
    let args: Vec<String> = env::args().collect();
    match capture(&args[1], &args[2]) {
        Ok(()) => (),
        Err(e) => println!("{:?}", e),
    }
}
