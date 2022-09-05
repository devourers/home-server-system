
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

fn handle_stream(mut stream: std::net::TcpStream){
    let buf_reader = std::io::BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let body: &String = &http_request[0];
    let sender: &String = &String::from("Client");
    let msg = utils::Message::new(body, sender, utils::MessageType::HelloMsg);
    process_message(&msg);
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
    let listener = std::net::TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();

        handle_stream(stream);
    }
}
