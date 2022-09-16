mod server;

fn main() {
    let srv = server::Server::init();
    println!("=====Server {0} started at {1}:{2}=====", &srv.name, &srv.adrs, &srv.port);
    println!("Avalaible commands:");
    for command in &srv.commands{
        println!("{}", command.0);
    }
    srv.run();
}
