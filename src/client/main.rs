mod client;

fn main(){
    println!("Client started... \n Form message as %message|code%");
    loop{
        let mut msg = String::new();
        std::io::stdin()
        .read_line(&mut msg)
        .expect("Failed to read line");
        client::send_request(&msg);
    }
}