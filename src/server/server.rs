
use futures::future::Map;
use notify_rust::Notification;
use utils;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};


pub struct Server{
    pub name: String,
    pub adrs: std::net::IpAddr,
    pub port: u16,
    pub commands: std::collections::HashMap<String, utils::ServerCommand>
}


impl Server{
    pub fn init() -> Self{
        let addr = local_ip_address::local_ip().unwrap();
        let name = ("home-server").to_string(); //TODO -- parse with json, wait for commands
        return Server{
            name: name,
            adrs: addr,
            port: utils::PORT,
            commands: std::collections::HashMap::new()
        }
    }

    pub fn run(&self){
        let adrsock = std::net::SocketAddr::new(self.adrs, self.port);
        let listener = std::net::TcpListener::bind(&adrsock).unwrap();
        for stream in listener.incoming(){
            self.handle_stream(stream);
        }
    }

    pub fn handle_stream(&self, stream: std::result::Result<std::net::TcpStream, std::io::Error>){
        let mut stream = stream.unwrap();
        loop{
            let mut buf_reader = std::io::BufReader::new(&mut stream);
            let mut client_line = String::new();
            let buff_res = buf_reader.read_line(&mut client_line);
            if buff_res.is_err() || matches!(client_line.as_str(), ""){
                break;
            }
            let body: &String = &client_line;
            let sender: &String = &String::from("Client");
            let msg = utils::Message::new(body, sender, utils::MessageType::HelloMsg);
            let response = "conn\n";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
            self.process_message(&msg);
        }
    
    }
    
    pub fn process_message(&self, msg : &utils::Message) -> i32{
        if matches!(msg.msg_type, utils::MessageType::HelloMsg){
            self.send_notif(&msg.body);
            return 0;
        } else if matches!(msg.msg_type, utils::MessageType::ShutdownMsg){ 
            return 0;
        }
        else{
            return -1;
        }
    }


    pub fn send_notif(&self, body: &String, ) {
        Notification::new()
        .summary("home-server")
        .body(body)
        .icon("dialog_information")
        .show().unwrap();
    }
}





