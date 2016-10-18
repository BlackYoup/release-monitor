use std::env;

pub struct MongoConfig {
    pub database: String,
    pub uri: String,
    pub port: u16
}

pub struct Config{
    pub mongo: MongoConfig
}

impl Config{
    pub fn new() -> Config{
        Config{
            mongo: MongoConfig::read_from_env()
        }
    }
}

impl MongoConfig{
    pub fn read_from_env() -> MongoConfig{
        let database = MongoConfig::get_env("MONGO_DATABASE");
        let uri = MongoConfig::get_env("MONGO_URI");
        let port = match MongoConfig::get_env("MONGO_PORT").parse::<u16>() {
            Ok(port) => port,
            Err(_) => panic!("Couldn't convert port to u16") // TODO: display port in error message
        };

        MongoConfig{
            database: database,
            uri: uri,
            port: port
        }
    }

    fn get_env(env: &str) -> String{
        match env::var_os(env) {
            Some(val) => {
                match val.into_string() {
                    Ok(v) => v,
                    Err(_) => panic!("Couldn't convert env {} to string", env)
                }
            },
            None => panic!("Missing {} environment variable", env)
        }
    }
}
