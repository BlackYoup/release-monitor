#[macro_use] extern crate bson;
extern crate hyper;
#[macro_use] extern crate lazy_static;
extern crate mongodb;
extern crate regex;
extern crate rustc_serialize;

mod models;
mod project;
mod config;
mod utils;

use regex::Regex;
use std::env;

use models::github::Github;
use project::*;
use config::Config;

fn main() {
    let config = Config::new();
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        help();
    } else {
        match args[1].to_string() {
            // TODO: ensure args[2] exists
            ref add if add == "add-project" => add_project(&args[2], &config),
            ref github if github == "github-import" => Github::import(&args[2], config),
            ref w if w == "watch" => watch(&config),
            _ => help()
        }
    }
}

fn help(){
    println!("Use: release-monitor <option>");
    println!("Available options:");
    println!("add <url>");
    println!("github-import <username>");
    println!("watch");
}

fn add_project(url: &String, config: &Config) {
    match match_project(&url, &config) {
        Some(project) => project.to_project().save(&config),
        None => panic!("Couldn't match project {}", url)
    };
}

fn match_project(url: &str, config: &Config) -> Option<Box<TProject>>{
    lazy_static!{
        static ref GITHUB_RE: Regex = Regex::new(r"github.com").unwrap();
    }

    if GITHUB_RE.is_match(url) {
        return Some(Box::new(Github::new(url.to_string(), config.clone())));
    } else{
        return None;
    }
}

fn watch(config: &Config) {
    let projects: Vec<Option<Project>> = Project::get_all_saved_projects(&config);

    println!("Analyzing {} projects", projects.len());

    for saved_project in projects {
        match saved_project {
            Some(saved_project) => {
                let mut project: Project = saved_project
                    .to_original(&config)
                    .expect("Couldn't transfer project to original")
                    .to_project();

                // TODO: what happens if there are no versions yet ?
                if Project::has_new_version(&saved_project, &project) {
                    println!("Project {} is more recent", project.name);
                } else {
                    println!("Project {} is the same", project.name);
                }

                let object_id = saved_project.object_id.unwrap();

                project.set_object_id(object_id);

                match project.save(&config) {
                    true => println!("Project {} updated", project.name),
                    false => println!("Couldn't update project {}", project.name)
                }
            },
            None => panic!("Couldn't parse project")
        }
    }
}
