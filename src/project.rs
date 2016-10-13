pub enum ProjectType{
    TGITHUB
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
    fn get_releases(&self) -> Vec<ProjectRelease>;
    fn get_name(&self) -> String;
    fn to_project(&self) -> Project;
    //fn get_name(&self) -> String;
    //fn has_new_version(&self) -> Bool;
    //fn get_last_release(&self) -> ProjectRelease;
}

impl Project{
    pub fn save(&self) -> bool {
        println!("Saving project {}", self.name);
        true
    }
}
