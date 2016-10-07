extern crate regex;
extern crate hyper;
extern crate rustc_serialize;

mod models;
mod project;

use regex::Regex;

use std::env;
use std::fs::File;
use std::io::Read;

use models::github::Github;
use project::Project;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Use: release-monitor <projects file>");
    } else {
        let urls = parse_projects(&args[1]);
        let github: Github = Github::new(urls.first().unwrap().clone());
        let releases = github.get_releases();

        for release in releases{
            println!("Release: {}", release.version);
        }
    }
}

// TODO
#[allow(unused_must_use)]
fn parse_projects(file_path: &String) -> Vec<String>{
    let mut file = File::open(file_path).unwrap();
    let mut buffer = String::new();
    let mut ret = Vec::new();

    file.read_to_string(&mut buffer);

    let lines = buffer.split('\n');
    let re = Regex::new(r"github.com").unwrap();

    for line in lines{
        if re.is_match(line){
            ret.push(line.to_string());
        }
    }

    return ret;
}
