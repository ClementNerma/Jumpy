use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, HashMap},
    path::Path,
};

pub struct Index {
    scored_entries: HashMap<String, u64>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            scored_entries: HashMap::new(),
        }
    }

    pub fn add_or_inc(
        &mut self,
        path: String,
        update_score: impl FnOnce(u64) -> u64,
        default_value: u64,
    ) -> Result<(), String> {
        if path.is_empty() {
            return Err("Please provide a valid path.".to_string());
        }

        if !Path::new(&path).exists() {
            return Err("Provided directory does not exist.".to_string());
        }

        // NOTE: we use 'dunce' to avoid (when possible) UNC paths which may
        //       result in unexpected behaviours
        let path = dunce::canonicalize(&path)
            .map_err(|e| format!("Failed to canonicalize path: {e}"))?
            .to_str()
            .ok_or_else(|| format!("Path contains invalid UTF-8 characters: {path}"))?
            .to_string();

        // Silently ignore root path
        if path == "/" {
            return Ok(());
        }

        match self.scored_entries.entry(path) {
            Entry::Occupied(mut entry) => {
                *entry.get_mut() = update_score(*entry.get());
            }
            Entry::Vacant(entry) => {
                entry.insert(default_value);
            }
        }

        Ok(())
    }

    pub fn add(&mut self, path: String) -> Result<(), String> {
        self.add_or_inc(path, |score| score, 1)
    }

    pub fn inc(&mut self, path: String, set_top: bool) -> Result<(), String> {
        self.add_or_inc(
            path,
            |score| {
                if set_top {
                    u64::MAX
                } else {
                    score.saturating_add(1)
                }
            },
            if set_top { u64::MAX / 2 } else { 1 },
        )
    }

    pub fn query_all(&self, query: &str, after: Option<&str>) -> Vec<IndexEntry> {
        let query = query.to_lowercase();

        let mut entries = self
            .scored_entries
            .iter()
            .filter_map(|(path, scope)| {
                Path::new(path)
                    .file_name()
                    .and_then(|filename| filename.to_str())
                    .filter(|filename| filename.to_lowercase().contains(&query))
                    .map(|_| (path, scope))
            })
            .map(IndexEntry::from)
            .collect::<Vec<_>>();

        entries.sort_by(|a, b| b.cmp(a));

        if let Some(after) = after {
            let index = entries.iter().position(|result| result.path == after);

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

    pub fn query_unchecked(&self, query: &str, after: Option<&str>) -> Option<&str> {
        self.query_all(query, after)
            .first()
            .map(|result| result.path)
    }

    pub fn query_checked(&mut self, query: &str, after: Option<&str>) -> Option<String> {
        let mut to_remove = vec![];

        let path = self
            .query_all(query, after)
            .iter()
            .find(|result| {
                if Path::new(result.path).exists() {
                    true
                } else {
                    to_remove.push(result.path.to_string());
                    false
                }
            })
            .map(|result| result.path.to_string())?;

        for path in to_remove {
            self.remove(&path).unwrap();
        }

        Some(path)
    }

    pub fn list(&self) -> Vec<&str> {
        let mut entries: Vec<_> = self.scored_entries.iter().map(IndexEntry::from).collect();

        entries.sort_by(|a, b| b.cmp(a));

        entries.iter().map(|result| result.path).collect()
    }

    pub fn remove(&mut self, path: &str) -> Result<(), &'static str> {
        match self.scored_entries.remove(path) {
            Some(_) => Ok(()),
            None => Err("Provided directory is not registered"),
        }
    }

    pub fn cleanup(&mut self) {
        let mut to_remove = vec![];

        for path in self.scored_entries.keys() {
            if !Path::new(path).is_dir() {
                to_remove.push(path.to_string());
            }
        }

        for path in to_remove {
            self.remove(&path).unwrap();
        }
    }

    pub fn export(&mut self) {
        println!("{}", self.encode());
    }

    pub fn clear(&mut self) {
        self.scored_entries = HashMap::new();
    }

    pub fn encode(&self) -> String {
        let mut entries = self
            .scored_entries
            .iter()
            .map(IndexEntry::from)
            .collect::<Vec<_>>();

        entries.sort_by(|a, b| b.cmp(a));

        entries
            .iter()
            .map(|entry| format!("{} {}", entry.score, entry.path))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn decode(input: &str) -> Result<Self, String> {
        let mut n = 0;
        let mut scored_entries = HashMap::new();

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

#[derive(PartialEq, Eq)]
pub struct IndexEntry<'a> {
    path: &'a str,
    score: u64,
}

impl<'a> From<(&'a String, &'a u64)> for IndexEntry<'a> {
    fn from(iter_entry: (&'a String, &'a u64)) -> Self {
        Self {
            path: iter_entry.0.as_str(),
            score: *iter_entry.1,
        }
    }
}

impl<'a> PartialOrd for IndexEntry<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for IndexEntry<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score).then_with(|| {
            self.path
                .cmp(other.path)
                .then_with(|| panic!("Got two directories with the same path: {}", other.path))
        })
    }
}
