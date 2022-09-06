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
        let ips: Vec<u8> = (0..255).map(|v| v).collect();
        let parsed_net: Vec<&str> = self.adrs.split(".").collect();
        let mut total_conns = 0;
        for ip in ips{
            println!("{}", &ip);
            let addr1: u8 = parsed_net[0].parse().unwrap();
            let addr2: u8 = parsed_net[1].parse().unwrap();
            let addr3: u8 = parsed_net[2].parse().unwrap();
            let curr_adr = std::net::SocketAddr::from(([addr1, addr2, addr3, ip], self.port.parse::<u16>().unwrap()));
            let curr_connect_result = std::net::TcpStream::connect_timeout(&curr_adr, std::time::Duration::from_millis(2));
            if curr_connect_result.is_ok() {
                let mut unwrapped_stream = curr_connect_result.unwrap();
                let wr_r = unwrapped_stream.write(&self.name.as_bytes());
                if wr_r.is_ok(){
                let fl_r = unwrapped_stream.flush();
                    if fl_r.is_ok(){
                        println!("written and flushed to server");
                    }
                }
                let _check_string = String::from("conn");
                let buf_reader = std::io::BufReader::new(&mut unwrapped_stream);
                let http_request: Vec<_> = buf_reader
                .lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();
                println!("created http request");
                if matches!(&http_request[0], _check_string){

                    println!("Connection established at {}", &curr_adr);
                    total_conns += 1;
                }
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

