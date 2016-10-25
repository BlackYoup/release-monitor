use hyper::{Client, Url};
use hyper::header::{UserAgent, Basic, Authorization};
use std::io::Read;
use std::str::from_utf8;

use config::*;
use project::*;
use rustc_serialize::json;
use utils::*;

#[derive(RustcDecodable, Clone)]
struct GithubCommit{
    sha: String,
    url: String
}

#[derive(RustcDecodable, Clone)]
pub struct GithubTag{
    pub name: String,
    zipball_url: String,
    tarball_url: String,
    commit: GithubCommit
}

#[derive(RustcDecodable, Clone)]
pub struct GithubProject{
    pub name: String
}

pub struct Github{
    pub tags: Option<Vec<GithubTag>>,
    pub url: String,
    pub project: Option<GithubProject>,
    pub api_url: Option<String>,
    pub config: Config
}

#[derive(RustcDecodable)]
pub struct GithubStarred{
    pub html_url: String
}

impl TProject for Github{
    fn to_project(&self) -> Project{
        Project{
            releases: self.get_releases(),
            name: self.get_name(),
            url: self.url.clone(),
            object_id: None,
            project_type: ProjectTypes::GITHUB
        }
    }
}

impl Github{
    // TODO: better handle Config
    pub fn new(url: String, config: Config) -> Github{
        let mut github = Github{
            url: url,
            tags: None,
            project: None,
            api_url: None,
            config: config
        };

        github.init();

        return github;
    }

    fn init(&mut self){
        self.api_url = Some(self.to_api_url());
        self.project = Some(self.get_github_project());
        self.tags = Some(self.get_github_releases());
    }

    // TODO
    #[allow(unused_must_use)]
    fn get_github_releases(&self) -> Vec<GithubTag>{
        let client = Client::new();

        let url = self.api_url.clone().unwrap() + "/tags";

        let mut res = client
            .get(Url::parse(&url).unwrap())
            .header(UserAgent(self.config.github.username.clone()))
            .header(Authorization(Basic{
                username: self.config.github.username.clone(),
                password: Some(self.config.github.token.clone())
            }))
            .send()
            .unwrap();

        let mut buffer = String::new();

        res.read_to_string(&mut buffer);

        json::decode(&buffer).unwrap()
    }

    // TODO
    #[allow(unused_must_use)]
    fn get_github_project(&self) -> GithubProject{
        let client = Client::new();

        let url = self.api_url.clone().unwrap();

        let mut res = client
            .get(Url::parse(&url).unwrap())
            .header(UserAgent(self.config.github.username.to_string()))
            .header(Authorization(Basic{
                username: self.config.github.username.clone(),
                password: Some(self.config.github.token.clone())
            }))
            .send()
            .unwrap();

        let mut buffer = String::new();

        res.read_to_string(&mut buffer);

        json::decode(&buffer).unwrap()
    }

    fn to_api_url(&self) -> String {
        return self.url
            .clone()
            .replace("github.com", "api.github.com/repos");
    }

    fn get_releases(&self) -> Vec<ProjectRelease>{
        let mut ret: Vec<ProjectRelease> = Vec::new();

        let tags = &self.tags.clone().unwrap();

        for tag in tags{
            ret.push(ProjectRelease::new(tag.name.clone(), None));
        }

        return ret;
    }

    fn get_name(&self) -> String {
        let project = &self.project.clone().unwrap();

        return project.name.clone();
    }

    // TODO
    #[allow(unused_must_use)]
    pub fn import(username: &str, config: Config) {
        let projects = Github::get_starred_projects(&username, &config, None).unwrap();
        let projects_count = projects.len();

        println!("Importing {} projects", projects_count);

        let mut count = 1;
        for project in projects{
            let p = Github::new(project.html_url, config.clone()).to_project();

            if !p.exists(&config) {
                println!("{}/{} Importing project {}", count, projects_count, p.name);
                p.save(&config);
            } else {
                println!("{}/{} Project {} already tracked, skipping it...", count, projects_count, p.name);
            }
            count = count + 1;
        }
    }

    #[allow(unused_must_use)]
    pub fn get_starred_projects(username: &str, config: &Config,
        next_url: Option<String>) -> Option<Vec<GithubStarred>>{
        let client = Client::new();
        let mut url = String::new();

        // TODO: find a better way
        if next_url.is_some() {
            url = next_url.unwrap();
        } else {
            url.push_str("https://api.github.com/users/");
            url.push_str(username);
            url.push_str("/starred");
        };

        println!("Calling URL {}", url);

        let mut res = client
            .get(Url::parse(&url).unwrap())
            .header(UserAgent(username.to_string()))
            .header(Authorization(Basic{
                username: config.github.username.clone(),
                password: Some(config.github.token.clone())
            }))
            .send()
            .unwrap();

        let mut buffer = String::new();
        res.read_to_string(&mut buffer);

        let tmp = res.headers.get_raw("Link").unwrap().first().unwrap();
        let next = extract_next_link(from_utf8(tmp.as_slice()).unwrap().to_string());

        match next {
            Some(next_url) => {
                let mut projects: Vec<GithubStarred> = json::decode(&buffer).unwrap();
                match Github::get_starred_projects(&username, &config, Some(next_url)) {
                    Some(mut p) => {
                        projects.append(&mut p);
                        Some(projects)
                    },
                    None => Some(projects)
                }
            },
            None => None
        }
    }
}
