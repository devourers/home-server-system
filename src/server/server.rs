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
        let mut buf_reader = std::io::BufReader::new(&mut stream);
        let mut client_line = String::new();
        let buff_res = buf_reader.read_line(&mut client_line).unwrap();
        let body: &String = &client_line;
        if body == "conn\n"{
            let mut response = "conn;".to_string();
            for command in &self.commands{
                response += &command.0;
                response += "|";
                response += &command.1.brief;
                response += ";";
            }
            response += "\n";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
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
        }
        if self.commands.contains_key(body) {
            let res = self.commands[body].exec();
            if res.0 == utils::CommandCode::Done{
                println!("Success: command {}", body);
            }
            else if res.0 == utils::CommandCode::Error{
                println!("Error: command {}", body);
            }
        }
        if self.commands.contains_key(splitted_body[0]){
            let res = self.commands[splitted_body[0]].exec();
            if res.0 == utils::CommandCode::Sync{
                let paths = utils::list_nested_dirs(res.1);
                let body_copy = splitted_body[0].clone().to_string();
                let sync_loc = res.1.to_string();
                let response_str = utils::form_sync_response(body_copy, &paths, &sync_loc);
                stream.write(&response_str.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        }
    }
}





