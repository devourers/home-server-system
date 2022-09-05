use notify_rust::Notification;
use utils;
use serde_json;
use serde;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Client{
    name : String,
    adrs : String,
    port : String
}

impl Client{
    pub fn init_from_json(file: String) -> Self{
        let data = std::fs::read_to_string(&file).expect("Unable to read file");
        let parsed_client: Client = serde_json::from_str(&data).unwrap();
        return parsed_client;
    }

    pub fn new(name : &String, adrs : &String, port : &String) -> Self{
        return Client{ 
            name : name.to_string(), 
            adrs : adrs.to_string(),
            port : port.to_string() };
    }

    pub fn scan_network(self){
        let ips: Vec<i32> = (0..255).map(|v| v).collect();
        let parsed_net: Vec<&str> = self.adrs.split(".").collect();
        let mut total_conns = 0;
        for ip in ips{
            println!("{}", &ip);
            let curr_adr = parsed_net[0].to_string()
            + "." + &parsed_net[1].to_string()
            + "." + &parsed_net[2].to_string()
            + "." + &ip.to_string() + ":" + &self.port;
            let curr_connect_result = std::net::TcpStream::connect(&curr_adr);
            if curr_connect_result.is_ok() {
                println!("Connection established at {}", &curr_adr);
                total_conns += 1;
            }
        }
        println!("Total {} devices discovered.", &total_conns.to_string());
    }

    pub fn to_string(self) -> String{
        let mut return_str = String::new();
        return_str.push_str("Client ");
        return_str.push_str(&self.name);
        return_str.push_str(", network ");
        return_str.push_str(&self.adrs);
        return_str.push_str(", port ");
        return_str.push_str(&self.port);
        return return_str;
    }
}

pub fn send_request(body : &String){
    let mut stream = std::net::TcpStream::connect("127.0.0.1:8080").unwrap();
    stream.write(body.as_bytes());
    stream.flush();
}

