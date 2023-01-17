use std::fmt::Display;

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub published: String,
    pub author: String,
    pub url: String,
    pub docs: Option<String>,
    pub source: Option<String>,
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}\nversion: {}\nauthor: {}\nurl: {}\ndocs: {:?}\nsource: {:?}",
            self.name, self.version, self.author, self.url, self.docs, self.source
        )
    }
}
