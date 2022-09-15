mod server;

fn main() {
    let srv = server::Server::init();
    println!("=====Server {0} started at {1}:{2}=====", &srv.name, &srv.adrs, &srv.port);
    srv.run();
}
