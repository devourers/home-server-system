use notify_rust::Notification;
use utils;
use serde_json;
use serde;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
pub struct Client{
    pub name: String,
    pub adrs: std::net::IpAddr,
    pub port: u16,
    pub addrs: Vec<std::net::SocketAddr>,
    pub curr_addr: std::net::SocketAddr,
    //pub curr_addr: std::net::TcpStream,
    pub is_connected: bool,
    pub outgoing_msg: String
}

impl Client{
    pub fn init() -> Self{
        let addr = local_ip_address::local_ip().unwrap();
        let name = ("home-client").to_string(); //TODO -- parse with json, wait for commands
        return Client{
            name: name,
            adrs: addr,
            port: utils::PORT,
            addrs: vec![],
            curr_addr: std::net::SocketAddr::from(([127, 0, 0, 1], 1337)),
            //curr_strm: std::net::TcpStream::connect_timeout(
            //    &std::net::SocketAddr::from(([127, 0, 0, 1], 1337)), std::time::Duration::from_millis(2)).unwrap(),
            is_connected: false,
            outgoing_msg: "".to_string()
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
                if server_line.eq(&_check_string){
                    println!("Connection established at {}", &curr_adr);
                    res_vec.push(curr_adr);
                    total_conns += 1;
                }
            }
        }
        println!("Total {} devices discovered.", &total_conns.to_string());
        self.addrs = res_vec;
    }

    pub fn connect(& mut self, &addr: &std::net::SocketAddr){
        self.curr_addr = addr;
        self.is_connected = true;
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

    pub fn send_request(&mut self, body : &String){
        let curr_connect_result = std::net::TcpStream::connect_timeout(&self.curr_addr, std::time::Duration::from_millis(2));
        let mut strm = curr_connect_result.unwrap();
        strm.write(body.as_bytes()).unwrap();
        strm.flush().unwrap();
    }

}




impl eframe::App for Client{
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame){
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Client GUI");
            ui.horizontal(|ui| {
                ui.label("Client name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            if ui.button("Scan networks").clicked() && self.addrs.len() == 0{
                ui.spinner();
                self.scan_network();  
            }
            let loop_addr = self.addrs.clone();
            if !self.is_connected{
                for addr in loop_addr{
                    let butt_str = ("Connect to ").to_string() + &addr.to_string();
                    if ui.button(butt_str).clicked(){
                        self.connect(&addr);
                    }
                }
            }
            if self.is_connected{
                ui.horizontal(|ui| {
                    ui.label("Send message");
                    ui.text_edit_singleline(&mut self.outgoing_msg);
                if self.outgoing_msg.len() > 0 && ctx.input().key_pressed(eframe::egui::Key::Enter) {
                    let copy_msg = self.outgoing_msg.clone();
                    self.send_request(&copy_msg);
                    self.outgoing_msg = "".to_string();
                }            
                });
            if ui.button("Disconnect").clicked(){
                self.is_connected = false;
                self.addrs = vec![];
                self.outgoing_msg = "".to_string();
            }
            }

        if ui.button("Quit").clicked(){
            _frame.close();
        }
        });


    }

    
}

 