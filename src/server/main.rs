mod server;

fn main() {
    println!("~~~~Server started atr 127.0.0.1:80~~~~");
    server::run_server();
}
