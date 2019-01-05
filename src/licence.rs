extern crate core;

use library::Library;
use reqwest::Response;
use regex::Regex;
use reqwest::Result;

pub struct Licence {
    pub name: String,
    pub licence_url: String
}

impl ToString for Licence {
    fn to_string(&self) -> String {
        return format!("Name: {}, url: {}", self.name, self.licence_url);
    }
}

pub fn find_licences(library: &Library) -> Vec<Licence> {
    let url = format!("https://mvnrepository.com/artifact/{}/{}", library.group_id, library.artifact_id);
    let body = reqwest::get(url.as_str())
                                 .and_then(|mut x| {x.text()}).ok()
                                 .map(|x| {get_licences_from_html(x.as_str())})
                                 .unwrap_or(vec![]);
    return body;
}

fn get_licences_from_html(html: &str) -> Vec<Licence> {
    let re = Regex::new(r#"<span class="b lic">(?P<licence>[\w\s\d.-_;':]+)</span>"#).unwrap();
    let library = re.captures_iter(html).map(|caps| {
        Licence {
            name: caps["licence"].to_string(),
            licence_url: "".to_string()
        }
    }).collect();
    return library;
}
