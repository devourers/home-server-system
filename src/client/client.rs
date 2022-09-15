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
    pub name: String,
    pub adrs: std::net::IpAddr,
    pub port: u16,
    pub addrs: Vec<std::net::SocketAddr>
}

impl Client{
    pub fn init() -> Self{
        let addr = local_ip_address::local_ip().unwrap();
        let name = ("home-client").to_string(); //TODO -- parse with json, wait for commands
        return Client{
            name: name,
            adrs: addr,
            port: utils::PORT,
            addrs: vec![]
        }
    }
    
    pub fn scan_network(&mut self){
        let mut res_vec: Vec<std::net::SocketAddr> = vec![];
        let ips: Vec<u8> = (0..255).map(|v| v).collect();
        let str_adrs = &self.adrs.to_string();
        let parsed_net: Vec<&str> = str_adrs.split(".").collect();
        let mut total_conns = 0;
        for ip in ips{
            let addr1: u8 = parsed_net[0].parse().unwrap();
            let addr2: u8 = parsed_net[1].parse().unwrap();
            let addr3: u8 = parsed_net[2].parse().unwrap();
            let curr_adr = std::net::SocketAddr::from(([addr1, addr2, addr3, ip], self.port));
            let curr_connect_result = std::net::TcpStream::connect_timeout(&curr_adr, std::time::Duration::from_millis(2));
            if curr_connect_result.is_ok() {
                let mut unwrapped_stream = curr_connect_result.unwrap();
                let _check_string = String::from("conn\n");
                let mut msg_to_server = self.name.to_string();
                msg_to_server.push_str("\n");
                let _wr_r = unwrapped_stream.write(&msg_to_server.as_bytes()).unwrap();
                let _fl_r = unwrapped_stream.flush().unwrap();
                let mut buf_reader = std::io::BufReader::new(&mut unwrapped_stream);
                let mut server_line = String::new();
                buf_reader.read_line(&mut server_line).unwrap();
                if matches!(&server_line, _check_string){
                    println!("Connection established at {}", &curr_adr);
                    res_vec.push(curr_adr);
                    total_conns += 1;
                }
            }
        }
        println!("Total {} devices discovered.", &total_conns.to_string());
        self.addrs = res_vec;
    }

    pub fn connect(&self, &addr: &std::net::SocketAddr) -> std::net::TcpStream{
        let mut curr_connect_result = std::net::TcpStream::connect_timeout(&addr, std::time::Duration::from_millis(2));
        if curr_connect_result.is_ok(){
            println!("Connected to server {}", &addr);
        }
        return curr_connect_result.unwrap();
    }

    pub fn to_string(&self) -> String{
        let mut return_str = String::new();
        return_str.push_str("Client ");
        return_str.push_str(&self.name);
        return_str.push_str(", network ");
        return_str.push_str(&self.adrs.to_string());
        return_str.push_str(", port ");
        return_str.push_str(&self.port.to_string());
        return return_str;
    }
}

impl eframe::App for Client{
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame){
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.name);
            ui.horizontal(|ui| {
                ui.label("Client name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            if ui.button("Scan networks").clicked() && self.addrs.len() == 0{
                ui.spinner();
                self.scan_network();  
            }
            for addr in &self.addrs{
                let butt_str = ("Connect to ").to_string() + &addr.to_string();
                if ui.button(butt_str).clicked(){
                    let mut stream = self.connect(&addr);
                    let mut outgoing_msg = String::new();
                    ui.text_edit_singleline(&mut outgoing_msg);
                    loop{
                        if outgoing_msg.len() > 0 && outgoing_msg.chars().nth(outgoing_msg.len() - 1).unwrap() == '\n' {
                            send_request(&mut stream, &outgoing_msg);
                            outgoing_msg = "".to_string();
                        }
                    } 
                }
            }
        });

    }

    
}

pub fn send_request(stream: &mut std::net::TcpStream, body : &String){
    stream.write(body.as_bytes());
    stream.flush();
}

 