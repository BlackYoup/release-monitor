use std::env;

#[derive(Clone)]
pub struct MongoConfig {
    pub database: String,
    pub uri: String,
    pub port: u16
}

#[derive(Clone)]
pub struct GithubConfig{
    pub username: String,
    pub token: String
}

#[derive(Clone)]
pub struct Config{
    pub mongo: MongoConfig,
    pub github: GithubConfig
}

impl Config{
    pub fn new() -> Config{
        Config{
            mongo: MongoConfig::read_from_env(),
            github: GithubConfig::read_from_env()
        }
    }
}

impl MongoConfig{
    pub fn read_from_env() -> MongoConfig{
        let database = Config::get_env("MONGO_DATABASE");
        let uri = Config::get_env("MONGO_URI");
        let port = match Config::get_env("MONGO_PORT").parse::<u16>() {
            Ok(port) => port,
            Err(_) => panic!("Couldn't convert port to u16") // TODO: display port in error message
        };

        MongoConfig{
            database: database,
            uri: uri,
            port: port
        }
    }
}

impl GithubConfig{
    pub fn read_from_env() -> GithubConfig{
        let username = Config::get_env("GITHUB_USERNAME");
        let token = Config::get_env("GITHUB_TOKEN");

        GithubConfig{
            username: username,
            token: token
        }
    }
}

impl Config{
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
