#[macro_use] extern crate bson;
extern crate hyper;
#[macro_use] extern crate lazy_static;
extern crate mongodb;
extern crate regex;
extern crate rustc_serialize;

mod models;
mod project;

use regex::Regex;

use std::env;
use std::fs::File;
use std::io::Read;

use models::github::Github;
use project::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Use: release-monitor <projects file>");
    } else {
        let projects: Vec<Box<TProject>> = parse_projects(&args[1]);

        for project in projects {
            let project: Project = project.to_project();
            project.save();
        }
    }
}

// TODO
#[allow(unused_must_use)]
fn parse_projects(file_path: &String) -> Vec<Box<TProject>>{
    let mut file = File::open(file_path).unwrap();
    let mut buffer = String::new();
    let mut ret: Vec<Box<TProject>> = Vec::new();

    file.read_to_string(&mut buffer);

    let lines = buffer.split('\n');

    for url in lines{
        let p = match_project(url.to_string());

        if p.is_some() {
            ret.push(p.unwrap());
        }
    }

    return ret;
}

fn match_project(url: String) -> Option<Box<TProject>>{
    lazy_static!{
        static ref GITHUB_RE: Regex = Regex::new(r"github.com").unwrap();
    }

    if GITHUB_RE.is_match(&url) {
        return Some(Box::new(Github::new(url)));
    } else{
        return None;
    }
}
