use std::net::TcpStream;
use std::io::prelude::*;

// http request over tcp
fn main(){
    println!("Test http request");

    let mut tcp_stream = TcpStream::connect("www.baidu.com:80")
        .expect("connection error");
    tcp_stream.write(b"GET / HTTP/1.0\n\n")
        .expect("get error");
    let mut buf = String::new();
    tcp_stream.read_to_string(&mut buf).expect("res error");

    println!("{}", buf);
}