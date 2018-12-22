extern crate core;
extern crate regex;
extern crate reqwest;

mod library;
mod licence;
mod licenced_library;

use std::io;
use std::io::BufRead;
use std::io::Lines;
use std::io::StdinLock;
use regex::Regex;
use library::Library;
use licence::find_licences;
use licence::Licence;
use licenced_library::LicencedLibrary;


fn get_direct_dependencies(lines: Lines<StdinLock>) -> Vec<Library> {
    let mut libs = lines.map(|line| { line.unwrap_or(String::from("")) })
                     .filter(|line| {line.starts_with("+")})
                     .map(|line| {line.replace("+---", "")})
                     .filter_map(Library::to_library)
                     .collect::<Vec<Library>>();
    libs.sort();
    libs.dedup();

    return libs;
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();

    let mut handle = stdin.lock();
    let lines_to_process = get_direct_dependencies(handle.lines());
    println!("{}", lines_to_process.iter().map(Library::to_string).collect::<Vec<String>>().join("\n"));

    let licenced_libraries = lines_to_process.iter().map(|line| {
        let licences = find_licences(line);
        return LicencedLibrary::new(line.clone(), licences);
    }).collect::<Vec<_>>();

    for lic in licenced_libraries {
        println!("{}", lic.to_string())
    }
}

