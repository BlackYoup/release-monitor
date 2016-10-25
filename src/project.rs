use bson::{Bson, Document};
use bson::oid::ObjectId;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

use config::Config;
use models::github::Github;
use regex::Regex;

pub enum ProjectTypes{
    GITHUB
}

// TODO: create getters / setters
pub struct Project{
    pub object_id: Option<ObjectId>,
    pub releases: Vec<ProjectRelease>,
    pub name: String,
    pub url: String,
    pub project_type: ProjectTypes
}

#[derive(PartialEq)]
pub enum ReleaseType{
    SAME,
    NEWER,
    OLDER
}

pub struct Version{
    major: Option<u16>,
    minor: Option<u16>,
    patch: Option<u16>,
    revision: Option<u16>
}

pub struct ProjectRelease{
    pub version: String,
    date: Option<String> // TODO: better type
}

impl ProjectRelease{
    pub fn new(version: String, date: Option<String>) -> ProjectRelease{
        ProjectRelease{
            version: version,
            date: date
        }
    }
}

pub trait TProject{
    fn to_project(&self) -> Project;
}

impl Project{
    // TODO: better return type
    #[allow(unused_must_use)]
    pub fn save(&self, config: &Config) -> bool {
        // TODO: env variables
        let client = Client::connect(&config.mongo.uri, config.mongo.port)
            .ok().expect("Couldn't connect to mongodb database");

        // TODO: env variables
        let collection = client.db(&config.mongo.database).collection("projects");
        let doc = self.get_document();

        if self.object_id.is_some() {
            let mut filter = Document::new();
            let object_id = self.object_id.clone().unwrap();
            filter.insert("_id".to_owned(), Bson::ObjectId(object_id));

            match collection.replace_one(filter, doc.clone(), None) {
                Ok(_) => true,
                Err(err) => panic!("Error when updating: {}", err)
            };
        } else {
            collection.insert_one(doc, None);
        }

        return true;
    }

    pub fn get_document(&self) -> Document{
        let mut doc = Document::new();

        doc.insert("name".to_owned(), Bson::String(self.name.clone()));
        doc.insert("url".to_owned(), Bson::String(self.url.clone()));
        doc.insert("type".to_owned(), Bson::String(self.get_project_type_str()));

        let mut releases: Vec<Bson> = Vec::new();
        for release in &self.releases{
            let mut release_doc = Document::new();
            release_doc.insert("version".to_owned(), Bson::String(release.version.clone()));
            releases.push(Bson::Document(release_doc));
        }

        doc.insert("releases", Bson::Array(releases));

        return doc;
    }

    pub fn get_all_saved_projects(config: &Config) -> Vec<Option<Project>>{
        let client = Client::connect(&config.mongo.uri, config.mongo.port)
            .ok().expect("Couldn't connect to mongodb database");

        let collection = client.db(&config.mongo.database).collection("projects");
        let find = Document::new();

        let results = collection.find(Some(find), None)
            .ok().expect("Failed to execute find");

        let mut res: Vec<Option<Project>> = Vec::new();

        for result in results{
            match result{
                Ok(doc) => res.push(Project::from_document(doc)),
                Err(_) => res.push(None)
            }
        }

        return res;
    }

    fn get_last_release(&self) -> Option<&ProjectRelease>{
        return self.releases.first();
    }

    pub fn has_new_version(project_old: &Project, project_new: &Project) -> bool{
        let version_old = &project_old.get_last_release();
        let version_new = &project_new.get_last_release();

        if version_old.is_none() {
            if version_new.is_some() {
                return true;
            } else {
                return false;
            }
        } else if version_new.is_none() {
            return false;
        } else {
            let v_old = Version::new(&version_old.unwrap().version);
            let v_new = Version::new(&version_new.unwrap().version);

            match Project::match_version(v_old, v_new) {
                ReleaseType::SAME => false,
                ReleaseType::NEWER => true,
                ReleaseType::OLDER => false
            }
        }
    }

    fn match_version_number(vo: Option<u16>, vn: Option<u16>) -> ReleaseType{
        if !vo.is_some() && vn.is_some() {
            return ReleaseType::NEWER;
        } else if vo.is_some() && !vn.is_some() {
            return ReleaseType::OLDER;
        } else if vo.is_some() && vn.is_some() {
            let von = vo.unwrap();
            let vnn = vn.unwrap();

            if von < vnn {
                return ReleaseType::NEWER;
            } else if von == vnn {
                return ReleaseType::SAME;
            } else if von > vnn {
                return ReleaseType::OLDER;
            }
        }

        return ReleaseType::SAME;
    }

    fn match_version(vo: Version, vn: Version) -> ReleaseType {
        let major = Project::match_version_number(vo.major, vn.major);
        if major != ReleaseType::SAME {
           return major;
        }

        let minor = Project::match_version_number(vo.minor, vn.minor);
        if minor != ReleaseType::SAME {
            return minor;
        }

        let patch = Project::match_version_number(vo.patch, vn.patch);
        if patch != ReleaseType::SAME {
            return patch;
        }

        let rev = Project::match_version_number(vo.revision, vn.revision);
        if rev != ReleaseType::SAME {
            return rev;
        }

        return ReleaseType::SAME;
    }

    pub fn set_object_id(&mut self, object_id: ObjectId) {
        self.object_id = Some(object_id);
    }

    fn from_document(doc: Document) -> Option<Project>{
        let object_id = match doc.get("_id") {
            Some(&Bson::ObjectId(ref object_id)) => object_id,
            _ => panic!("Couldn't get item Id in Database")
        };

        let name = match doc.get("name") {
            Some(&Bson::String(ref name)) => name,
            _ => panic!("Couldn't get project name")
        };

        let url = match doc.get("url") {
            Some(&Bson::String(ref url)) => url,
            _ => panic!("Couldn't get project url")
        };

        let project_type = match doc.get("type") {
            Some(&Bson::String(ref project_type)) => project_type,
            _ => panic!("Couldn't get project type")
        };

        let b_releases = match doc.get("releases") {
            Some(&Bson::Array(ref releases)) => releases,
            _ => panic!("Couldn't get project releases")
        };

        let mut releases: Vec<ProjectRelease> = Vec::new();

        for b_release in b_releases{
            let release = match b_release {
                &Bson::Document(ref release) => {
                    match release.get("version") {
                        Some(&Bson::String(ref version)) => version,
                        _ => panic!("Couldn't get project release version")
                    }
                },
                _ => panic!("Couldn't get project release")
            };

            releases.push(ProjectRelease::new(release.clone(), None));
        }

        return Some(Project{
            object_id: Some(object_id.clone()),
            url: url.clone(),
            name: name.clone(),
            releases: releases,
            project_type: Project::get_project_type_enum(&project_type)
        });
    }

    fn get_project_type_str(&self) -> String{
        match self.project_type {
            ProjectTypes::GITHUB => "Github".to_string()
        }
    }

    fn get_project_type_enum(project_type: &str) -> ProjectTypes{
        match project_type {
            "Github" => ProjectTypes::GITHUB,
            _ => panic!("Unknown project type {}", project_type)
        }
    }

    pub fn to_original(&self, config: &Config) -> Option<Box<TProject>>{
        match self.project_type {
            ProjectTypes::GITHUB => Some(Box::new(Github::new(self.url.clone(), config.clone())))
        }
    }

    pub fn exists(&self, config: &Config) -> bool{
        let mut find = Document::new();
        find.insert("name".to_owned(), Bson::String(self.name.clone()));
        find.insert("url".to_owned(), Bson::String(self.url.clone()));

        let client = Client::connect(&config.mongo.uri, config.mongo.port)
            .ok().expect("Couldn't connect to mongodb database");

        let collection = client.db(&config.mongo.database).collection("projects");

        let result = collection.find_one(Some(find), None).ok().expect("Couldn't find_one project");

        match result {
            Some(_) => true,
            None => false
        }
    }
}

impl Version{
    pub fn new(version: &String) -> Version{
        let re = Regex::new("^[A-Za-z]*-?").unwrap();
        let _version = re.replace(version, "");
        let numbers: Vec<&str> = _version.split('.').collect();

        let major = match numbers.get(0) {
            Some(x) => {
                match x.parse::<u16>() {
                    Ok(y) => Some(y),
                    Err(_) => Some(0)
                }
            },
            None => panic!("Couldn't get major version number from {}", version)
        };

        let minor = match numbers.get(1) {
            Some(x) => {
                match x.parse::<u16>() {
                    Ok(y) => Some(y),
                    Err(_) => None
                }
            },
            None => None
        };

        let patch = match numbers.get(2) {
            Some(x) => {
                match x.parse::<u16>() {
                    Ok(y) => Some(y),
                    _ => None
                }
            },
            None => None
        };

        let revision = match numbers.last() {
            Some(last) => {
                let revs: Vec<&str> = last.splitn(1, "-").collect();
                match revs.get(1) {
                    Some(x) => match x.parse::<u16>() {
                        Ok(y) => Some(y),
                        Err(_) => None
                    },
                    None => None
                }
            },
            None => None
        };

        return Version{
            major: major,
            minor: minor,
            patch: patch,
            revision: revision
        };
    }
}
