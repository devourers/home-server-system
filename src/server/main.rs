mod server;

fn main() {
    let srv = server::Server::init();
    //let srv = server::Server::init_from_json(("server_config.json").to_string());
    println!("~~~~Server {0} started at {1}:{2}~~~~", &srv.name, &srv.adrs, &srv.port);
    srv.run();
}
