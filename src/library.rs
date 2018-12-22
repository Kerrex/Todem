use regex::Regex;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Library {
    pub group_id: String,
    pub artifact_id: String,
    pub version: String
}



impl ToString for Library {
    fn to_string(&self) -> String {
        return format!("Grupa: {}, artefakt: {}, wersja: {}", self.group_id, self.artifact_id, self.version);
    }
}

impl Library {
    pub fn to_library(library: String) -> Option<Library> {
        let re = Regex::new(r"(?P<group_id>\S+):(?P<artifact_id>.+):(?P<version>[\w\d._-]+)").unwrap();
        let library = re.captures(library.as_str()).map(|caps| {
            Library {
                group_id: caps["group_id"].to_string(),
                artifact_id: caps["artifact_id"].to_string(),
                version: caps["version"].to_string()
            }
        });
        return library;
    }
}

