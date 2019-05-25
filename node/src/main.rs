use std::env;
use std::io;
use std::net::{TcpStream, UdpSocket};
use std::io::Write;

#[derive(Debug)]
struct Node {
    id: u16,
    port: u16,
    edges: Box<Vec<(u16, u16)>>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if &args.len() < &2 {
        println!("Missing command line arguments!");
        std::process::exit(1);
    }

    let node_s: Node = parse_args(args);

    // Must be a better way
    let mut udp_socket = UdpSocket::bind(("localhost", node_s.port + node_s.id)).expect("Failed to create UDP connection");
    let mut stream = TcpStream::connect(("localhost", node_s.id)).expect("Failed to make TCP connection");

    send_edges(&mut stream, node_s.edges).expect("Failed to send edges.");

    // Send data about self
    // Recv routing table
    // Send packets to other nodes on UDP
}


fn parse_args(args: Vec<String>) -> Node {
    let id: u16 = args[1].parse().expect("First argument was not a valid u16!");
    let port: u16 = args[2].parse().expect("Second argument was not a valid u16!"); 
    let mut node_s = Node { id: id, port: port, edges: Box::new(vec![]), };

    for arg in &args[3..] {
        let xs: Vec<u16> = arg.split(':').map(|x| x.parse::<u16>().expect("Invalid command line args")).collect();
        dbg!(&xs);
        node_s.edges.push((xs[0], xs[1]));
    }

    node_s
}

fn send_edges(stream: &mut TcpStream, edges: Box<Vec<(u16, u16)>>) -> io::Result<()> {
    for (id, weight) in edges.iter() {
        write!(stream, "{} {} ", id, weight)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_args() {
        let mut a: Vec<String> = vec![];
        a.push(String::from("filename"));
        a.push(String::from("10"));
        a.push(String::from("10000"));
        a.push(String::from("1:8"));

        let mut n = Node { id: 10, port: 10000, edges: Box::new(Vec::new()) };
        n.edges.push((1, 8));

        assert_eq!(parse_args(a).id, 10);
    
    }
}
