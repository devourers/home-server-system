mod client;

fn main(){
    let clnt = client::Client::init();
    let options = eframe::NativeOptions::default();
    eframe::run_native("Client", options, Box::new(|_cc| Box::new(clnt)));
}