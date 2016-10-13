use rustc_serialize::json;
use project::*;

use hyper::Client;
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
    pub project: Option<GithubProject>
}

impl TProject for Github{
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

    fn to_project(&self) -> Project{
        Project{
            releases: self.get_releases(),
            name: self.get_name(),
            url: self.url.clone(),
            project_type: ProjectType::TGITHUB
        }
    }
}

impl Github{
    pub fn new(url: String) -> Github{
        let mut github = Github{
            url: url,
            tags: None,
            project: None
        };

        github.init();

        return github;
    }

    fn init(&mut self){
        self.project = Some(self.get_github_project());
        self.tags = Some(self.get_github_releases());
    }

    // TODO
    #[allow(unused_must_use)]
    fn get_github_releases(&self) -> Vec<GithubTag>{
        let client = Client::new();

        let mut api_url = String::new();
        api_url.push_str(&self.url);
        api_url.push_str("/tags");

        println!("Calling {}", api_url);

        let mut res = client
            .get(&api_url)
            .header(UserAgent("BlackYoup".to_string()))
            .send()
            .unwrap();

        let mut buffer = String::new();

        res.read_to_string(&mut buffer);

        json::decode(&buffer).unwrap()
    }

    fn get_github_project(&self) -> GithubProject{
        let client = Client::new();

        let mut res = client
            .get(&self.url)
            .header(UserAgent("BlackYoup".to_string()))
            .send()
            .unwrap();

        let mut buffer = String::new();

        res.read_to_string(&mut buffer);

        json::decode(&buffer).unwrap()
    }
}
