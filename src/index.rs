use std::{
    collections::{btree_map::Entry, BTreeMap},
    path::{Component, Path},
};

use anyhow::{bail, Context, Result};

pub struct Index {
    scored_entries: BTreeMap<String, u64>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            scored_entries: BTreeMap::new(),
        }
    }

    pub fn canonicalize(path: impl AsRef<str>) -> Result<String> {
        let path = path.as_ref();

        // NOTE: we use 'dunce' to avoid (when possible) UNC paths which may
        //       result in unexpected behaviours
        let path = dunce::canonicalize(path)
            .with_context(|| format!("Failed to canonicalize path: {path}"))?
            .to_str()
            .with_context(|| format!("Path contains invalid UTF-8 characters: {path}"))?
            .to_string();

        Ok(path)
    }

    pub fn add_or_inc(
        &mut self,
        path: String,
        update_score: impl FnOnce(u64) -> u64,
        default_value: u64,
    ) -> Result<()> {
        if path.is_empty() {
            bail!("Please provide a valid path.");
        }

        if !Path::new(&path).exists() {
            bail!("Provided directory does not exist.");
        }

        let path = Self::canonicalize(path)?;

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

    pub fn add(&mut self, path: String) -> Result<()> {
        self.add_or_inc(path, |score| score, 1)
    }

    pub fn inc(&mut self, path: String, set_top: bool) -> Result<()> {
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

        let mut results = self
            .scored_entries
            .iter()
            .filter(|(path, _)| {
                Path::new(path)
                    .file_name()
                    .map(|filename| filename.to_str().unwrap())
                    .filter(|filename| filename.to_lowercase().contains(&query))
                    .is_some()
            })
            .map(IndexEntry::from)
            .collect::<Vec<_>>();

        let skip = match after {
            Some(after) => match results.iter().position(|result| result.path == after) {
                Some(index) => {
                    results.drain(0..=index);

                    self.scored_entries
                        .iter()
                        .position(|(path, _)| path == after)
                        .unwrap()
                        + 1
                }

                None => 0,
            },

            None => 0,
        };

        results.extend(
            self.scored_entries
                .iter()
                .skip(skip)
                .filter(move |(path, _)| {
                    if let Some(after) = after {
                        if path.starts_with(after) {
                            return false;
                        }
                    }

                    Path::new(path).components().any(|component| {
                        if let Component::Normal(component) = component {
                            if let Some(component) = component.to_str() {
                                return component.to_lowercase().contains(&query);
                            }
                        }

                        false
                    })
                })
                .map(IndexEntry::from),
        );

        results
    }

    /// Perform a query without checking if the result directories still exist
    pub fn query_unchecked(&self, query: &str, after: Option<&str>) -> Option<&str> {
        self.query_all(query, after)
            .first()
            .map(|result| result.path)
    }

    /// Perform a query but remove directory entries which don't exist anymore
    pub fn query_checked(&mut self, query: &str, after: Option<&str>) -> Option<String> {
        let mut to_remove = vec![];

        let path = self
            .query_all(query, after)
            .into_iter()
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
            self.remove_canonicalized(&path).unwrap();
        }

        Some(path)
    }

    pub fn iter(&self) -> impl Iterator<Item = IndexEntry> {
        self.scored_entries.iter().map(IndexEntry::from)
    }

    pub fn remove_canonicalized(&mut self, path: &str) -> Result<()> {
        match self.scored_entries.remove(path) {
            Some(_) => Ok(()),
            None => bail!("Provided directory is not registered"),
        }
    }

    pub fn cleanup(&mut self) {
        let to_remove = self
            .scored_entries
            .keys()
            .filter(|path| !Path::new(path).is_dir())
            .cloned()
            .collect::<Vec<_>>();

        for path in to_remove {
            self.remove_canonicalized(&path).unwrap();
        }
    }

    pub fn export(&mut self) {
        println!("{}", self.encode());
    }

    pub fn clear(&mut self) {
        self.scored_entries = BTreeMap::new();
    }

    pub fn encode(&self) -> String {
        self.scored_entries
            .iter()
            .map(IndexEntry::from)
            .map(|entry| format!("{} {}", entry.score, entry.path))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn decode(input: &str) -> Result<Self> {
        let mut n = 0;
        let mut scored_entries = BTreeMap::new();

        for line in input.lines() {
            n += 1;

            let split = line
                .find(' ')
                .with_context(|| format!("Missing whitespace delimited at line {n}"))?;

            let score = &line[0..split]
                .parse::<u64>()
                .with_context(|| format!("Failed to parse score at line {n}"))?;

            let path = &line[split + 1..];

            if scored_entries.contains_key(path) {
                bail!("Duplicate directory entry at line {n}: {path}");
            }

            scored_entries.insert(path.to_string(), *score);
        }

        Ok(Self { scored_entries })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct IndexEntry<'a> {
    pub path: &'a str,
    pub score: u64,
}

impl<'a> From<(&'a String, &'a u64)> for IndexEntry<'a> {
    fn from(iter_entry: (&'a String, &'a u64)) -> Self {
        Self {
            path: iter_entry.0.as_str(),
            score: *iter_entry.1,
        }
    }
}
