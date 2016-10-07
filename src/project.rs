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

pub trait Project<T>{
    fn new(url: String) -> T;
    //fn get_last_release(&self) -> ProjectRelease;
    fn get_releases(&self) -> Vec<ProjectRelease>;
}
