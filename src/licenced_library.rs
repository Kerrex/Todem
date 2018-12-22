use licence::Licence;
use library::Library;

pub struct LicencedLibrary {
    pub library: Library,
    pub licences: Vec<Licence>
}

impl LicencedLibrary {
    pub fn new(library: Library, licences: Vec<Licence>) -> LicencedLibrary {
        return LicencedLibrary {
            library,
            licences
        }
    }
}

impl ToString for LicencedLibrary {
    fn to_string(&self) -> String {
        let licences = self.licences.iter().map(Licence::to_string).collect::<Vec<String>>().join(", ");
        return format!("Library: {}, licences: {}", self.library.to_string(), licences);
    }
}