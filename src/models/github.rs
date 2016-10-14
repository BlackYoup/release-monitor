// TODO: env var
const GITHUB_NAME: &'static str = "BlackYoup";

use rustc_serialize::json;
use project::*;

use hyper::{Client, Url};
use hyper::header::UserAgent;

use std::io::Read;

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
    pub api_url: Option<String>
}

impl TProject for Github{
    fn to_project(&self) -> Project{
        Project{
            releases: self.get_releases(),
            name: self.get_name(),
            url: self.url.clone(),
            project_type: ProjectType::GITHUB
        }
    }
}

impl Github{
    pub fn new(url: String) -> Github{
        let mut github = Github{
            url: url,
            tags: None,
            project: None,
            api_url: None
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

        println!("Got url: {}", url);

        let mut res = client
            .get(Url::parse(&url).unwrap())
            .header(UserAgent("BlackYoup".to_string()))
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
            .header(UserAgent(GITHUB_NAME.to_string()))
            .send()
            .unwrap();

        let mut buffer = String::new();

        res.read_to_string(&mut buffer);

        json::decode(&buffer).unwrap()
    }

    fn to_api_url(&self) -> String {
        return self.url.clone().replace("github.com", "api.github.com");
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
}
