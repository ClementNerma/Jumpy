use std::{
    collections::{btree_map::Entry, BTreeMap},
    fs,
    path::Path,
};

pub struct Index {
    scored_entries: BTreeMap<String, u64>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            scored_entries: BTreeMap::new(),
        }
    }

    pub fn add_or_inc(&mut self, path: String, inc_if_exists: bool) -> Result<(), String> {
        if path.is_empty() {
            return Err("Please provide a valid path.".to_string());
        }

        if !Path::new(&path).exists() {
            return Err("Provided directory does not exist.".to_string());
        }

        let path = fs::canonicalize(&path)
            .map_err(|e| format!("Failed to canonicalize path: {e}"))?
            .to_str()
            .ok_or_else(|| format!("Path contains invalid UTF-8 characters: {path}"))?
            .to_string();

        match self.scored_entries.entry(path) {
            Entry::Occupied(mut entry) => {
                if inc_if_exists {
                    *entry.get_mut() += 1;
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(1);
            }
        }

        Ok(())
    }

    pub fn add(&mut self, path: String) -> Result<(), String> {
        self.add_or_inc(path, false)
    }

    pub fn inc(&mut self, path: String) -> Result<(), String> {
        self.add_or_inc(path, true)
    }

    pub fn query_all(&self, query: &str, after: Option<&str>) -> Vec<(&String, &u64)> {
        let query = query.to_lowercase();

        let mut entries = self
            .scored_entries
            .iter()
            .filter(|(path, _)| {
                Path::new(path)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_lowercase()
                    .contains(&query)
            })
            .collect::<Vec<_>>();

        entries.sort_by(|a, b| b.1.cmp(a.1));

        if let Some(after) = after {
            let index = entries.iter().position(|entry| entry.0 == after);

            if let Some(index) = index {
                let count = entries.len();
                entries = entries
                    .into_iter()
                    .skip(if index + 1 < count { index + 1 } else { 0 })
                    .collect();
            }
        }

        entries
    }

    pub fn query_unchecked(&self, query: &str, after: Option<&str>) -> Option<&String> {
        self.query_all(query, after).get(0).map(|result| result.0)
    }

    pub fn query_checked(&mut self, query: &str, after: Option<&str>) -> Option<String> {
        let mut to_remove = vec![];

        let path = self
            .query_all(query, after)
            .iter()
            .filter(|(path, _)| {
                if Path::new(path).exists() {
                    true
                } else {
                    to_remove.push(String::clone(path));
                    false
                }
            })
            .next()
            .map(|(path, _)| String::clone(path))?;

        for path in to_remove {
            self.remove(&path).unwrap();
        }

        Some(path)
    }

    pub fn list(&self) -> Vec<&String> {
        let mut entries: Vec<_> = self.scored_entries.iter().collect();

        entries.sort_by(|a, b| b.1.cmp(a.1));

        entries.iter().map(|entry| entry.0).collect()
    }

    pub fn remove(&mut self, path: &str) -> Result<(), &'static str> {
        match self.scored_entries.remove(path) {
            Some(_) => Ok(()),
            None => Err("Provided directory is not registered"),
        }
    }

    pub fn clear(&mut self) {
        self.scored_entries = BTreeMap::new();
    }

    pub fn encode(&self) -> String {
        self.scored_entries
            .iter()
            .map(|(path, score)| format!("{} {}", *score, path))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn decode(input: &str) -> Result<Self, String> {
        let mut n = 0;
        let mut scored_entries = BTreeMap::new();

        for line in input.lines() {
            n += 1;

            let split = line
                .find(' ')
                .ok_or_else(|| format!("Missing whitespace delimited at line {n}"))?;

            let score = &line[0..split]
                .parse::<u64>()
                .map_err(|e| format!("Failed to parse score at line {n}: {e}"))?;

            let path = &line[split + 1..];

            if scored_entries.contains_key(path) {
                return Err(format!("Duplicate directory entry at line {n}: {path}"));
            }

            scored_entries.insert(path.to_string(), *score);
        }

        Ok(Self { scored_entries })
    }
}
