use notify_rust::Notification;
use utils;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

struct Client{
    name : String,
    adrs : String,
}

impl Client{
    pub fn new(name : &String, adrs : &String) -> Self{
        return Client{ 
            name : name.to_string(), 
            adrs : adrs.to_string() };
    }
}

pub fn send_request(body : &String){
    let mut stream = std::net::TcpStream::connect("127.0.0.1:8080").unwrap();
    stream.write(body.as_bytes());
    stream.flush();
}