
use notify_rust::Notification;
use utils;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Server{
    pub name: String,
    pub adrs: String,
    pub port: String
}


impl Server{
    pub fn init_from_json(file: String) -> Self{
        let data = std::fs::read_to_string(&file).expect("Unable to read file");
        let parsed_server: Server = serde_json::from_str(&data).unwrap();
        return parsed_server;
    }

    pub fn new(name: String, adrs: String, port: String) -> Self{
        return Server{            
            name : name.to_string(), 
            adrs : adrs.to_string(),
            port : port.to_string() }
    }

    pub fn run(&self){
        let listener = std::net::TcpListener::bind("192.168.43.164:8080").unwrap();
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
            stream.write(response.as_bytes());
            stream.flush();
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





