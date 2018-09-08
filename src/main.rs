//! # Rust UA Matcher
//! Simple Rust application which extracts the Browser(the name an the used version)
//! from the user-agent string.

extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::{Regex, RegexBuilder};
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::io::{self, Write};

/// Represents a Browser
#[derive(Debug)]
pub struct Browser<'t> {
    display_name: &'static str,
    version: &'t str,
}

impl<'t> PartialEq for Browser<'t> {
    fn eq(&self, other: &Browser) -> bool {
        self.display_name == other.display_name && self.version == other.version
    }
}

impl<'t> Display for Browser<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Browser: {} Version: {}",
            self.display_name, self.version
        )
    }
}

lazy_static! {
    static ref FIREFOX_REGEX: Regex = {
        RegexBuilder::new(r"firefox/([\d\.]+)")
            .case_insensitive(true)
            .build()
            .unwrap()
    };
    static ref EDGE_REGEX: Regex = {
        RegexBuilder::new(r"edge/(\d{2}\.\d+)")
            .case_insensitive(true)
            .build()
            .unwrap()
    };
    static ref CHROME_REGEX: Regex = {
        RegexBuilder::new(r"(?:chromium|chrome)/([\d\.]+)")
            .case_insensitive(true)
            .build()
            .unwrap()
    };
    static ref BROWSER_LIST: HashMap<&'static str, &'static Regex> = {
        let mut m = HashMap::new();

        m.insert("Firefox", &*FIREFOX_REGEX);
        m.insert("Edge", &*EDGE_REGEX);
        m.insert("Chrome", &*CHROME_REGEX);

        m
    };
}

fn main() {
    let title = "User-Agent matcher";

    println!("{}", title);
    println!("{}", "-".repeat(title.chars().count()));
    println!("");
    println!("Enter an User-Agent string to get the version string.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut user_agent = String::new();

        io::stdin().read_line(&mut user_agent).unwrap();

        let user_agent: &str = user_agent.trim();

        println!("User-Agent: {}", user_agent);

        if let Some(ref browser) = get_browser(user_agent) {
            println!("{}", browser);
        } else {
            println!("No result :/");
        }
    }
}

/// Iterates on every known browser (precompiled-)regex
/// and checks whether it matches. If so, it returns a [Browser](struct.browser.html)
/// object.
pub fn get_browser(user_agent: &str) -> Option<Browser> {
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

#[cfg(test)]
mod tests {
    use super::{get_browser, Browser};

    #[test]
    fn test_detects_browser() {
        let user_agent_string =
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:64.0) Gecko/20100101 Firefox/64.0";

        let result = get_browser(user_agent_string);

        assert!(result.is_some());

        assert_eq!(
            result.unwrap(),
            Browser {
                display_name: "Firefox",
                version: "64.0"
            }
        )
    }
}
