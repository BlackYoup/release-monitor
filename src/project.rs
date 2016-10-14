use bson::{Bson, Document};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

pub enum ProjectType{
    GITHUB
}

pub struct Project{
    pub releases: Vec<ProjectRelease>,
    pub name: String,
    pub url: String,
    pub project_type: ProjectType
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
    pub fn save(&self) -> bool {
        // TODO: env variables
        let client = Client::connect("127.0.0.1", 27017)
            .ok().expect("Couldn't connect to mongodb database");

        // TODO: env variables
        let collection = client.db("release_monitor").collection("projects");
        let doc = self.get_document();

        collection.insert_one(doc.clone(), None)
            .ok().expect("Failed to insert document");

        return true;
    }

    pub fn get_document(&self) -> Document{
        let mut doc = Document::new();

        doc.insert("name".to_owned(), Bson::String(self.name.clone()));
        doc.insert("url".to_owned(), Bson::String(self.url.clone()));
        doc.insert("url".to_owned(), Bson::String("YOLO".to_string()));

        let mut releases: Vec<Bson> = Vec::new();
        for release in &self.releases{
            let mut release_doc = Document::new();
            release_doc.insert("version".to_owned(), Bson::String(release.version.clone()));
            releases.push(Bson::Document(release_doc));
        }

        doc.insert("versions", Bson::Array(releases));

        return doc;
    }
}
