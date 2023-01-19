use std::fmt::Display;

#[derive(Debug)]
pub struct Package {
    pub groups: Vec<Vec<(String, Option<String>)>>,
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.groups
                .iter()
                .map(|g| {
                    g.iter()
                        .filter(|(_, v)| v.is_some())
                        .map(|(k, v)| format!("{}: {}", k.to_lowercase(), v.clone().unwrap()))
                        .collect::<Vec<String>>()
                        .join("\n")
                })
                .collect::<Vec<String>>()
                .join("\n\n")
        )
    }
}
