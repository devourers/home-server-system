mod client;

fn main(){
    println!("Client started... \n Form message as %message|code%");
    let clnt = client::Client::init_from_json(("client_config.json").to_string());
    let servers = clnt.scan_network();
    println!("{}", &clnt.to_string());
    let mut stream = clnt.connect(servers);
    loop{
        let mut msg = String::new();
        std::io::stdin()
        .read_line(&mut msg)
        .expect("Failed to read line");
        client::send_request(&mut stream, &msg);
    }
}