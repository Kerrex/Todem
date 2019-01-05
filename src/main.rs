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

fn print_head_section() {
    println!("<head>");
    println!("<link href=\"https://stackpath.bootstrapcdn.com/bootstrap/4.2.1/css/bootstrap.min.css\" rel=\"stylesheet\" integrity=\"sha384-GJzZqFGwb1QTTN6wy59ffF1BuGJpLSa9DkKMp0DgiMDm4iYMj70gZWKYbI706tWS\" crossorigin=\"anonymous\">");
    println!("</head>");
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();

    let mut handle = stdin.lock();
    let lines_to_process = get_direct_dependencies(handle.lines());

    let licenced_libraries = lines_to_process.iter().map(|line| {
        let licences = find_licences(line);
        return LicencedLibrary::new(line.clone(), licences);
    }).collect::<Vec<_>>();


    println!("<html>");
    print_head_section();

    println!("<body>");
    println!("<div>");
    println!("<table class=\"table table-striped\">");
    println!("<thead><tr>");
    println!("<th scope=\"col\">Grupa</th><th scope=\"col\">Artefakt</th><th scope=\"col\">Wersja</th><th scope=\"col\">Licencja</th>");
    println!("</thead>");

    println!("<tbody>");
    for lic in licenced_libraries {
        println!("<tr>");
        println!("<td>{}</td>", lic.library.group_id);
        println!("<td>{}</td>", lic.library.artifact_id);
        println!("<td>{}</td>", lic.library.version);
        println!("<td>{}</td>", lic.licences.iter().map(|x| x.name.clone()).collect::<Vec<_>>().join(", "));
        println!("</tr>");
    }
    println!("</tbody>");
    println!("</table>");
    println!("</div>");
    println!("</body>");
    println!("</html>");
}

