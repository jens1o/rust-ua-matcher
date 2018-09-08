extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::Write;

#[derive(Debug)]
struct Browser<'t> {
    display_name: &'static str,
    version: &'t str,
}

lazy_static! {
    static ref FIREFOX_REGEX: Regex = { Regex::new(r"firefox/([\d\.]+)").unwrap() };
    static ref EDGE_REGEX: Regex = { Regex::new(r"edge/(\d{2}\.\d+)").unwrap() };
    static ref BROWSER_LIST: HashMap<&'static str, &'static Regex> = {
        let mut m = HashMap::new();

        m.insert("Firefox", &*FIREFOX_REGEX);
        m.insert("Edge", &*EDGE_REGEX);

        m
    };
}

fn main() {
    let mut user_agent = String::new();
    let title = "User-Agent matcher";

    println!("{}", title);
    println!("{}", "-".repeat(title.chars().count()));
    println!("");
    println!("Enter an User-Agent string to get the version string.");

    print!("> ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut user_agent).unwrap();

    let user_agent: &str = user_agent.trim();

    println!("User-Agent: {}", user_agent);

    if let Some(ref browser) = get_browser(user_agent) {
        println!("Found a result: {:?}", browser);
    } else {
        println!("No result :/");
    }
}

fn get_browser(user_agent: &str) -> Option<Browser> {
    for (display_name, regex) in BROWSER_LIST.iter() {
        if let Some(ref matches) = regex.captures(user_agent) {
            return Some(Browser {
                display_name,
                version: matches.get(1).unwrap().as_str(),
            });
        }
    }

    None
}
