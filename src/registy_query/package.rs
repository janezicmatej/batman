use std::fmt::Display;

#[derive(Debug)]
pub struct Package {
    pub metadata: Vec<(String, Option<String>)>,
    pub urls: Vec<(String, Option<String>)>,
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let metadata: String = self
            .metadata
            .iter()
            .filter(|(_, v)| v.is_some())
            .map(|(k, v)| format!("{}: {}", k.to_lowercase(), v.clone().unwrap()))
            .intersperse("\n".to_string())
            .collect();
        let urls: String = self
            .urls
            .iter()
            .filter(|(_, v)| v.is_some())
            .map(|(k, v)| format!("{}: {}", k.to_lowercase(), v.clone().unwrap()))
            .intersperse("\n".to_string())
            .collect();

        write!(f, "{metadata}\n\n{urls}")
    }
}
