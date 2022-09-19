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
        let name = ("home-server").to_string();
        let data = std::fs::read_to_string("server_commands.json");
        if data.is_ok() { //if we have commands then add them
            let cmmnds: std::collections::HashMap<String, utils::ServerCommand> = serde_json::from_str(&data.unwrap()).unwrap();
            return Server{
                name: name,
                adrs: addr,
                port: utils::PORT,
                commands: cmmnds
            }
        } else{
            return Server{
                name: name,
                adrs: addr,
                port: utils::PORT,
                commands: std::collections::HashMap::new()
            }
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
            let mut msg_type: utils::MessageType = utils::MessageType::HelloMsg;
            let body: &String = &client_line;
            let splitted_body: Vec<&str> = body.split('|').collect();
            if  splitted_body[0] == "file"{
                let mut name = "";
                if std::env::consts::OS == "windows"{
                    name = splitted_body[1].split('\\').last().unwrap();
                }
                else{
                    name = splitted_body[1].split('/').last().unwrap();
                } //is it win only? test
                let file_content = base64::decode(&splitted_body[2]).unwrap();
                let file_content = String::from_utf8(file_content).unwrap();
                std::fs::write(&name, &file_content).unwrap();
                msg_type = utils::MessageType::CommandMsg;
            }
            let sender: &String = &String::from("Client");
            if self.commands.contains_key(body){
                self.commands[body].exec();
            }
            let mut response = "conn;".to_string();
            for command in &self.commands{
                response += &command.0;
                response += "|";
                response += &command.1.brief;
                response += ";";
                msg_type = utils::MessageType::CommandMsg;
            }
            response += "\n";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
            let msg = utils::Message::new(body, sender, msg_type);
            self.process_message(&msg);
        }
    
    }
    
    pub fn process_message(&self, msg : &utils::Message) -> i32{
        if matches!(msg.msg_type, utils::MessageType::HelloMsg){
            self.send_notif(&msg.body);
            return 0;
        } else if matches!(msg.msg_type, utils::MessageType::CommandMsg){ 
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





