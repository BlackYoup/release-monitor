use regex::Regex;

pub struct Version{
    pub major: Option<u16>,
    pub minor: Option<u16>,
    pub patch: Option<u16>,
    pub revision: Option<u16>
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
