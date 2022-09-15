mod client;

fn main(){
    println!("Client started...");
    let clnt = client::Client::init();
    let options = eframe::NativeOptions::default();
    eframe::run_native("Client", options, Box::new(|_cc| Box::new(clnt)));
    //let servers = clnt.scan_network();
    //println!("{}", &clnt.to_string());
    //let mut stream = clnt.connect(servers);
    //loop{
    //    let mut msg = String::new();
    //    std::io::stdin()
    //    .read_line(&mut msg)
    //    .expect("Failed to read line");
    //    client::send_request(&mut stream, &msg);
    //}
}