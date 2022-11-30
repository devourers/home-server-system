use utils;
use serde_json;
use serde;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, str::FromStr
};

#[derive(Debug)]
pub struct Client{
    pub name: String,
    pub adrs: std::net::IpAddr,
    pub port: u16,
    pub addrs: Vec<std::net::SocketAddr>,
    pub curr_addr: std::net::SocketAddr,
    pub curr_adrr_str: String,
    //pub curr_addr: std::net::TcpStream,
    pub is_connected: bool,
    pub outgoing_msg: String,
    pub server_commands: std::collections::HashMap<String, String>,
    pub client_sync_locations: std::collections::HashMap<String, String>
}

impl Client{
    pub fn init() -> Self{
        let addr = local_ip_address::local_ip().unwrap();
        let name = ("home-client").to_string(); //TODO -- parse with json, wait for commands
        let path = "client_config.json";
        let data = std::fs::read_to_string(&path);
        let mut sync_locs: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        if data.is_ok(){
            sync_locs = serde_json::from_str(&data.unwrap().as_str()).unwrap();
        }
        return Client{
            name: name,
            adrs: addr,
            port: utils::PORT,
            addrs: vec![],
            curr_addr: std::net::SocketAddr::from(([127, 0, 0, 1], 1337)),
            curr_adrr_str: "".to_string(),
            //curr_strm: std::net::TcpStream::connect_timeout(
            //    &std::net::SocketAddr::from(([127, 0, 0, 1], 1337)), std::time::Duration::from_millis(2)).unwrap(),
            is_connected: false,
            outgoing_msg: "".to_string(),
            server_commands: std::collections::HashMap::new(),
            client_sync_locations: sync_locs,
        }
    }
    
    pub fn scan_network(&mut self){
        let mut res_vec: Vec<std::net::SocketAddr> = vec![];
        let ips: Vec<u8> = (0..255).map(|v| v).collect();
        let str_adrs = &self.adrs.to_string();
        let parsed_net: Vec<&str> = str_adrs.split(".").collect();
        for ip in ips{
            let addr1: u8 = parsed_net[0].parse().unwrap();
            let addr2: u8 = parsed_net[1].parse().unwrap();
            let addr3: u8 = parsed_net[2].parse().unwrap();
            let curr_adr = std::net::SocketAddr::from(([addr1, addr2, addr3, ip], self.port));
            let curr_connect_result = std::net::TcpStream::connect_timeout(&curr_adr, std::time::Duration::from_millis(50));
            if curr_connect_result.is_ok() {
                let mut unwrapped_stream = curr_connect_result.unwrap();
                let _check_string = String::from("conn");
                let mut msg_to_server = _check_string.to_string();
                msg_to_server.push_str("\n");
                let _wr_r = unwrapped_stream.write(&msg_to_server.as_bytes()).unwrap();
                let _fl_r = unwrapped_stream.flush().unwrap();
                let mut buf_reader = std::io::BufReader::new(&mut unwrapped_stream);
                let mut server_line = String::new();
                buf_reader.read_line(&mut server_line).unwrap();
                let splitted_server_line: Vec<&str> = server_line.split(";").collect();
                if splitted_server_line[0].eq(&_check_string){
                    if splitted_server_line.len() > 2{
                        for i in 1..splitted_server_line.len(){
                            let curr_command: Vec<&str> = splitted_server_line[i].split('|').collect();
                            if curr_command.len() == 2{
                                self.server_commands.insert(curr_command[0].to_string(), curr_command[1].to_string());
                            }
                        }
                    }
                    res_vec.push(curr_adr);
                }
            }
        }
        self.addrs = res_vec;
    }

    pub fn connect_direct(&mut self) -> bool{
        let addr = std::net::SocketAddr::from_str(&self.curr_adrr_str);
        if addr.is_ok(){
            let mut curr_adr = addr.unwrap();
            let curr_connect_result = std::net::TcpStream::connect_timeout(&curr_adr, std::time::Duration::from_millis(50));
            if curr_connect_result.is_ok() {
                let mut unwrapped_stream = curr_connect_result.unwrap();
                let _check_string = String::from("conn");
                let mut msg_to_server = _check_string.to_string();
                msg_to_server.push_str("\n");
                let _wr_r = unwrapped_stream.write(&msg_to_server.as_bytes()).unwrap();
                let _fl_r = unwrapped_stream.flush().unwrap();
                let mut buf_reader = std::io::BufReader::new(&mut unwrapped_stream);
                let mut server_line = String::new();
                buf_reader.read_line(&mut server_line).unwrap();
                let splitted_server_line: Vec<&str> = server_line.split(";").collect();
                if splitted_server_line[0].eq(&_check_string){
                    if splitted_server_line.len() > 2{
                        for i in 1..splitted_server_line.len(){
                            let curr_command: Vec<&str> = splitted_server_line[i].split('|').collect();
                            if curr_command.len() == 2{
                                self.server_commands.insert(curr_command[0].to_string(), curr_command[1].to_string());
                            }
                        }
                    }
                }
                self.connect(&curr_adr);
                return true;
            }
            else{
                return false;
            }
        }
        else{
            return false;
        }
    }

    pub fn connect(& mut self, &addr: &std::net::SocketAddr){
        self.curr_addr = addr;
        self.is_connected = true;
    }

    pub fn send_request(&mut self, body : &String) -> std::net::TcpStream{
        let curr_connect_result = std::net::TcpStream::connect_timeout(&self.curr_addr, std::time::Duration::from_millis(300));
        let mut strm = curr_connect_result.unwrap();
        strm.write(&body.as_bytes()).unwrap();
        strm.flush().unwrap();
        return strm;
    }

    pub fn send_file(&mut self, file_path: &String){
        let file = std::fs::read(&file_path).unwrap();
        let curr_connect_result = std::net::TcpStream::connect_timeout(&self.curr_addr, std::time::Duration::from_millis(20));
        let mut strm = curr_connect_result.unwrap();
        let first_string = "file|".to_string() + &file_path + &"|".to_string();
        let mut msg: Vec<u8> = first_string.as_bytes().to_vec();
        let file = base64::encode(&file);
        let mut file = file.as_bytes().to_vec();
        msg.append(&mut file);
        strm.write(&msg).unwrap();
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
            if !self.is_connected{

                if ui.button("Scan network").clicked() && self.addrs.len() == 0{
                    ui.spinner();
                    self.scan_network();  
                }
                ui.horizontal(|ui| {
                    ui.label("Connect to known IP ");
                    ui.text_edit_singleline(&mut self.curr_adrr_str);
                    if self.curr_addr.to_string().len() > 0 && ctx.input().key_pressed(eframe::egui::Key::Enter){
                        if !self.connect_direct(){
                            self.curr_adrr_str = "".to_string();
                        }
                    }
                
                });
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
                ui.horizontal(|ui|{
                    ui.heading("Server commands");
                    let loop_comms = self.server_commands.clone();
                    for command in loop_comms{
                        let com_but = ui.button(&command.0);
                        let com_but = com_but.on_hover_text(&command.1);
                        if com_but.clicked(){
                            if command.0.contains("sync"){
                                let files = utils::list_nested_dirs(&self.client_sync_locations[&command.0]);
                                let command0 = command.0.clone();
                                let msg = utils::form_sync_response(command0, &files, &self.client_sync_locations[&command.0]);
                                let mut curr_stream = self.send_request(&msg);
                                let mut buf_reader = std::io::BufReader::new(&mut curr_stream);
                                let mut server_line = String::new();
                                buf_reader.read_line(&mut server_line).unwrap();
                                utils::compare_dirs(&msg, &server_line, &std::path::Path::new(&self.client_sync_locations[&command.0]));
                            }
                            else{
                                self.send_request(&command.0);
                            }
                        }
                    }
                });
                if ui.button("Disconnect").clicked(){
                    self.is_connected = false;
                    self.addrs = vec![];
                    self.outgoing_msg = "".to_string();
                }
                if ui.button("Send file").clicked(){
                    let mut picked_path = String::new();
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        picked_path = Some(path.display().to_string()).unwrap();
                    }
                    self.send_file(&picked_path);
                }
            }

        if ui.button("Quit").clicked(){
            _frame.close();
        }
        });


    }

    
}

 