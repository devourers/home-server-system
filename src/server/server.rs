
use notify_rust::Notification;
use utils;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn send_notif(body: &String, ) {
    Notification::new()
    .summary("home-server")
    .body(body)
    .icon("dialog_information")
    .show().unwrap();
}


fn handle_stream(stream: std::result::Result<std::net::TcpStream, std::io::Error>){
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
        process_message(&msg);
    }

}

fn process_message(msg : &utils::Message) -> i32{
    if matches!(msg.msg_type, utils::MessageType::HelloMsg){
        send_notif(&msg.body);
        return 0;
    } else if matches!(msg.msg_type, utils::MessageType::ShutdownMsg){ 
        return 0;
    }
    else{
        return -1;
    }
}

pub fn run_server(){
    let listener = std::net::TcpListener::bind("192.168.1.66:8080").unwrap();
    for stream in listener.incoming(){
        handle_stream(stream);
    }
}
