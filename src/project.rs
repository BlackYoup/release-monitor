use bson::{Bson, Document};
use bson::oid::ObjectId;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::results::UpdateResult;

use config::Config;

// TODO: create getters / setters
pub struct Project{
    pub object_id: Option<ObjectId>,
    pub releases: Vec<ProjectRelease>,
    pub name: String,
    pub url: String,
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

            collection.update_one(filter, doc, None);
        } else {
            collection.insert_one(doc.clone(), None);
        }

        return true;
    }

    pub fn get_document(&self) -> Document{
        let mut doc = Document::new();

        doc.insert("name".to_owned(), Bson::String(self.name.clone()));
        doc.insert("url".to_owned(), Bson::String(self.url.clone()));

        let mut releases: Vec<Bson> = Vec::new();
        for release in &self.releases{
            let mut release_doc = Document::new();
            release_doc.insert("version".to_owned(), Bson::String(release.version.clone()));
            releases.push(Bson::Document(release_doc));
        }

        doc.insert("releases", Bson::Array(releases));

        return doc;
    }

    pub fn get_saved_project(&self, config: &Config) -> Option<Project>{
        // TODO: re-use same client accross execution
        let client = Client::connect(&config.mongo.uri, config.mongo.port)
            .ok().expect("Couldn't connect to mongodb database");

        let collection = client.db(&config.mongo.database).collection("projects");

        let mut find = Document::new();
        find.insert("name".to_owned(), Bson::String(self.name.clone()));
        find.insert("url".to_owned(), Bson::String(self.url.clone()));

        let result = collection.find_one(Some(find), None)
            .ok().expect("Failed to execute find");

       match result{
            Some(res) => {
                let object_id = match res.get("_id") {
                    Some(&Bson::ObjectId(ref object_id)) => object_id,
                    _ => panic!("Couldn't get item Id in Database")
                };

                let name = match res.get("name") {
                    Some(&Bson::String(ref name)) => name,
                    _ => panic!("Couldn't get project name")
                };

                let url = match res.get("url") {
                    Some(&Bson::String(ref url)) => url,
                    _ => panic!("Couldn't get project url")
                };

                let b_releases = match res.get("releases") {
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
                    releases: releases
                });
            },
            None => None
        }
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
            println!("Not some 1");
            return ReleaseType::NEWER;
        } else if vo.is_some() && !vn.is_some() {
            println!("Not some 2");
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
}

impl Version{
    pub fn new(version: &String) -> Version{
        let numbers: Vec<&str> = version.split('.').collect();

        let major = match numbers.get(0) {
            Some(x) => {
                match x.parse::<u16>() {
                    Ok(y) => Some(y),
                    _ => panic!("Couldn't format major {} from {}", x, version)
                }
            },
            None => panic!("Couldn't get major version number from {}", version)
        };

        let minor = match numbers.get(1) {
            Some(x) => {
                match x.parse::<u16>() {
                    Ok(y) => Some(y),
                    _ => None
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
                let revs: Vec<&str> = last.split('-').collect();

                match revs.len() {
                    2 => {
                        match revs.get(1) {
                            Some(x) => match x.parse::<u16>() {
                                Ok(y) => Some(y),
                                _ => panic!("Couldn't format revision for {}", version)
                            },
                            None => None
                        }
                    },
                    0...1 => None,
                    _ => panic!("Multiple revisions ({}) for same project", version)
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
