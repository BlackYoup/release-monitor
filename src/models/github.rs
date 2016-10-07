use rustc_serialize::json;
use project::{Project, ProjectRelease};

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

pub struct Github{
    tags: Option<Vec<GithubTag>>,
    url: String
}

impl Project<Github> for Github{
    fn new(url: String) -> Github{
        let mut github = Github{
            url: url,
            tags: None
        };

        github.init();

        return github;
    }

    fn get_releases(&self) -> Vec<ProjectRelease>{
        let mut ret: Vec<ProjectRelease> = Vec::new();

        let tags = &self.tags.clone().unwrap();

        for tag in tags{
            ret.push(ProjectRelease::new(tag.name.clone(), None));
        }

        return ret;
    }
}

impl Github{
    fn init(&mut self){
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
}
