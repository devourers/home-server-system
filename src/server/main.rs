mod server;

fn main() {
    let my_local_ip = local_ip_address::local_ip().unwrap();
    println!("Local ip is {}", my_local_ip);
    let srv = server::Server::init_from_json(("server_config.json").to_string());
    println!("~~~~Server {0} started at {1}:{2}~~~~", &srv.name, &srv.adrs, &srv.port);
    srv.run();
}
