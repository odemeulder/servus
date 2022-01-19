use std::error::Error;

pub struct Config {
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        
        // if args.len() < 2 {
        //     return Err("not enough arguments");
        // }

        // let filename = args[1].clone();

        Ok(Config { })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    println!("Servus Webserver");
  
    Ok(())
}