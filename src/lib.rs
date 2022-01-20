use std::error::Error;

mod server;

pub struct Config {
}

impl Config {
    pub fn new(_args: &[String]) -> Result<Config, &str> {
        
        // if args.len() < 2 {
        //     return Err("not enough arguments");
        // }

        Ok(Config { })
    }
}

pub fn run(_config: Config) -> Result<(), Box<dyn Error>> {

    println!("Servus Webserver");

    let server = server::Server::new();
    server.serve().expect("error");

    Ok(())
}