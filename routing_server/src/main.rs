use std::env;
use std::io::Read;
// use std::io::prelude::*;
use std::net::TcpListener;
use std::str::from_utf8;

#[derive(Debug)]
struct Node {
    id: u16,
    edges: Box<Vec<(u16, u16)>>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if &args.len() < &3 {
        println!("Missing command line arguments!");
        std::process::exit(1);
    }

    dbg!(&args);

    let xs: Vec<u16> = args[1..].iter().map(|x| x.parse::<u16>().expect("Illegal command line arg")).collect();
    let (port, _n): (u16, u16) = (xs[0], xs[1]);

    let listener = TcpListener::bind(("localhost", port)).expect("Failed to create TCP listener");
    let (mut socket, _addr) = listener.accept().expect("Failed to accept socket");
    let mut buffer: Vec<u8> = vec![];
    socket.read_to_end(&mut buffer).expect("Failed read");

    let s = from_utf8(&buffer).expect("Failed conversion to UTF-8");

    println!("{}", s);

}
