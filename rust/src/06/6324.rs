use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    const DEFAULT: &str = "<default>";

    for (i, input) in buf.lines().enumerate().skip(1) {
        let (protocol, url) = input.split_once("://").unwrap();
        let (host, path) = url.split_once('/').unwrap_or((url, DEFAULT));
        let (hostname, port) = host.split_once(':').unwrap_or((host, DEFAULT));

        println!("URL #{i}");
        println!("Protocol = {protocol}");
        println!("Host     = {hostname}");
        println!("Port     = {port}");
        println!("Path     = {path}");
        println!("");
    }
}
